use std::{
    cmp::{
        max,
        min,
    },
    collections::HashMap,
    str::FromStr,
};

pub fn day_5() -> String {
    let input: Vec<Line> =
        input_to_lines(&std::fs::read_to_string("assets/day5.input").unwrap());
    format!("Part 1: {}\nPart 2: {}", day5a(input.clone()), day5b(input))
}

fn input_to_lines(input: &str) -> Vec<Line> {
    input
        .split('\n')
        .filter_map(|s| s.parse::<String>().ok())
        .map(|l| Line::from_str(&l).unwrap())
        .collect()
}

#[derive(Debug, Clone)]
struct Line {
    x0: isize,
    x1: isize,
    y0: isize,
    y1: isize,
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<String> = s.split(' ').map(str::to_string).collect();
        let zeroes: Vec<isize> = parts[0]
            .split(',')
            .map(|num| num.parse::<isize>().unwrap())
            .collect();
        let ones: Vec<isize> = parts[2]
            .split(',')
            .map(|num| num.parse::<isize>().unwrap())
            .collect();
        Ok(Self {
            x0: zeroes[0],
            x1: ones[0],
            y0: zeroes[1],
            y1: ones[1],
        })
    }
}

impl Line {
    fn coords(&self, diagonals_included: bool) -> Vec<(isize, isize)> {
        let mut line = Vec::new();
        if self.x0 == self.x1 {
            for y in min(self.y0, self.y1)..max(self.y0, self.y1) + 1 {
                line.push((self.x0, y))
            }
        } else if self.y0 == self.y1 {
            for x in min(self.x0, self.x1)..max(self.x0, self.x1) + 1 {
                line.push((x, self.y0))
            }
        } else if diagonals_included {
            let dx = (self.x1 - self.x0).signum();
            let dy = (self.y1 - self.y0).signum();
            for i in 0..(self.x1 - self.x0).abs() + 1 {
                line.push((self.x0 + i * dx, self.y0 + i * dy))
            }
        }
        line
    }
}

fn day5_base(lines: Vec<Line>, diagonals_included: bool) -> usize {
    let mut counts: HashMap<(isize, isize), isize> = HashMap::new();
    let mut coords = Vec::new();
    for line in lines {
        coords.extend(line.coords(diagonals_included).iter())
    }
    for coord in coords {
        counts
            .entry(coord)
            .and_modify(|n| *n += 1)
            .or_insert(1isize);
    }
    counts
        .values()
        .filter(|val| **val != 1isize)
        .count()
}

/// 5280
fn day5a(lines: Vec<Line>) -> usize { day5_base(lines, false) }

/// 16716
fn day5b(lines: Vec<Line>) -> usize { day5_base(lines, true) }

#[cfg(test)]
mod tests {
    use crate::day5::{
        day5a,
        day5b,
        input_to_lines,
    };

    const RAW_INPUT: &str = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn day5a_example() { assert_eq!(day5a(input_to_lines(RAW_INPUT)), 5) }
    #[test]
    fn day5b_example() { assert_eq!(day5b(input_to_lines(RAW_INPUT)), 12) }
}
