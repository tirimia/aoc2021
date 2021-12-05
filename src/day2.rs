// MAYBE: move from go method to AddAssign
use std::str::FromStr;

use crate::read_lines;

pub fn day_2() -> String {
    let input: Vec<Move> = read_lines("assets/day2.input")
        .iter()
        .map(|l| Move::from_str(l).unwrap())
        .collect();
    format!("Part 1: {}\nPart 2: {}", day2a(input.clone()), day2b(input))
}

/// 1714680
fn day2a(moves: Vec<Move>) -> usize {
    moves
        .iter()
        .fold(SubmarinePartA::default(), |final_pos, step| {
            final_pos.go(*step)
        })
        .final_position()
}

/// 1963088820
fn day2b(moves: Vec<Move>) -> usize {
    moves
        .iter()
        .fold(SubmarinePartB::default(), |final_pos, step| {
            final_pos.go(*step)
        })
        .final_position()
}

trait Submarine {
    fn final_position(&self) -> usize;
    fn go(self, m: Move) -> Self;
}

impl Submarine for SubmarinePartA {
    fn final_position(&self) -> usize { self.x * self.y }

    fn go(self, m: Move) -> Self {
        let mut new = self;
        match m.direction {
            Direction::Forward => new.x += m.steps,
            Direction::Down => new.y += m.steps,
            Direction::Up => new.y -= m.steps,
        }
        new
    }
}

impl Submarine for SubmarinePartB {
    fn final_position(&self) -> usize { self.x * self.y }
    fn go(self, m: Move) -> Self {
        let mut new = self;
        match m.direction {
            Direction::Forward => {
                new.x += m.steps;
                new.y += new.aim * m.steps
            }
            Direction::Down => new.aim += m.steps,
            Direction::Up => new.aim -= m.steps,
        }
        new
    }
}

#[derive(Default)]
struct SubmarinePartA {
    x: usize,
    y: usize,
}

#[derive(Default)]
struct SubmarinePartB {
    x: usize,
    y: usize,
    aim: usize,
}

#[derive(Clone, Copy)]
struct Move {
    direction: Direction,
    steps: usize,
}

impl FromStr for Move {
    type Err = Day2Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<&str>>();
        Ok(Self {
            direction: parts[0].parse::<Direction>()?,
            steps: parts[1].parse::<usize>()?,
        })
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = Day2Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            "forward" => Ok(Self::Forward),
            _ => Err(Day2Error::DirectionParseError(s.to_string())),
        }
    }
}

#[derive(Debug)]
enum Day2Error {
    DirectionParseError(String),
    StepParseError(String),
}

impl From<std::num::ParseIntError> for Day2Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::StepParseError(e.to_string())
    }
}

impl std::fmt::Display for Day2Error {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Day2Error::DirectionParseError(s) => {
                write!(f, "Could not parse direction : {}", s)
            }
            Day2Error::StepParseError(s) => {
                write!(f, "Could not parse step: {}", s)
            }
        }
    }
}
