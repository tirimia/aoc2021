mod verify;
use anyhow::{
    anyhow,
    Error,
    Result,
};
use std::{
    collections::HashSet,
    iter::FromIterator,
    str::FromStr,
};

type Dots = HashSet<[usize; 2]>;

pub struct Puzzle {
    dots: Dots,
    folds: Vec<Fold>,
}

pub enum PuzzleOutput {
    Dots(usize),
    Code(String),
}

impl Iterator for Puzzle {
    type Item = PuzzleOutput;

    fn next(&mut self) -> Option<Self::Item> {
        if 1 >= self.dots.len() {
            return None;
        }
        if let Some(fold) = self.folds.pop() {
            match fold {
                Fold::Vertical(x) => {
                    self.dots =
                        HashSet::from_iter(self.dots.iter().map(|dot| {
                            if dot[0] > x {
                                [x * 2 - dot[0], dot[1]]
                            } else {
                                *dot
                            }
                        }))
                }
                Fold::Horizontal(y) => {
                    self.dots =
                        HashSet::from_iter(self.dots.iter().map(|dot| {
                            if dot[1] > y {
                                [dot[0], y * 2 - dot[1]]
                            } else {
                                *dot
                            }
                        }));
                }
            }
        } else {
            // get max x
            // get max y
            let max_x = self.dots.iter().map(|d| d[0]).max().unwrap();
            let max_y = self.dots.iter().map(|d| d[1]).max().unwrap();
            let mut code = String::new();
            for y in 0..=max_y {
                for x in 0..=max_x {
                    code.push(if self.dots.contains(&[x, y]) {
                        '#'
                    } else {
                        ' '
                    });
                }
                code.push('\n');
            }
            return Some(PuzzleOutput::Code(code));
        }
        Some(PuzzleOutput::Dots(self.dots.len()))
    }
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dots_raw, folds_raw) = s
            .split_once("\n\n")
            .ok_or(anyhow!("missing double newline"))?;
        let dots = dots_raw
            .split('\n')
            .into_iter()
            .map(|d| {
                d.split(',')
                    .filter_map(|num| num.parse::<usize>().ok())
                    .collect::<Vec<usize>>()
            })
            .map(|v| [v[0], v[1]])
            .collect();
        let folds = folds_raw
            .split('\n')
            .filter_map(|l| Fold::from_str(l).ok())
	    // Reversing so we can pop() out during iteration
            .rev()
            .collect();
        Ok(Self { dots, folds })
    }
}

enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

impl FromStr for Fold {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, val) = s
            .split(' ')
            .last()
            .ok_or(anyhow!("couldn't get last part of fold line"))?
            .split_once('=')
            .ok_or(anyhow!("can't split key fold chunk by ="))?;
        let value = val.parse::<usize>()?;
        match direction {
            "y" => Ok(Self::Horizontal(value)),
            "x" => Ok(Self::Vertical(value)),
            _ => Err(anyhow!("Unknown direction {}", direction)),
        }
    }
}

pub fn day13a(puzzle: &str) -> usize {
    if let Some(PuzzleOutput::Dots(d)) = Puzzle::from_str(puzzle)
        .unwrap()
        .into_iter()
        .next()
    {
        d
    } else {
        0usize
    }
}
pub fn day13b(puzzle: &str) -> String {
    // Still quite ugly, but works^TM
    if let Some(PuzzleOutput::Code(output)) = Puzzle::from_str(puzzle)
        .unwrap()
        .into_iter()
        .find(|output| {
            match output {
                PuzzleOutput::Dots(_) => false,
                PuzzleOutput::Code(_) => true,
            }
        })
    {
        output
    } else {
        "".to_string()
    }
}
