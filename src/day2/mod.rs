mod verify;
use std::str::FromStr;

fn day2_base<S: Submarine>(moves: Vec<Move>, sub: S) -> usize {
    moves
        .iter()
        .fold(sub, |final_pos, step| final_pos.go(*step))
        .final_position()
}

pub fn day2a(moves: Vec<Move>) -> usize {
    day2_base(moves, SubmarinePartA::default())
}

pub fn day2b(moves: Vec<Move>) -> usize {
    day2_base(moves, SubmarinePartB::default())
}

pub fn parse_moves<S: AsRef<str>>(input: &[S]) -> Vec<Move> {
    input
        .iter()
        .map(|instr| Move::from_str(instr.as_ref()).unwrap())
        .collect()
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
pub struct Move {
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
pub enum Day2Error {
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
