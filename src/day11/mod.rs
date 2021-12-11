mod verify;
use std::collections::HashSet;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

type Octopi = [usize; 100];

trait FlattenedIdx {
    fn idx(&self) -> usize;
    fn from(point: usize) -> Self;
}

impl FlattenedIdx for (usize, usize) {
    fn idx(&self) -> usize { self.0 + self.1 * WIDTH }
    fn from(point: usize) -> Self { (point % WIDTH, point / WIDTH) }
}

fn get_neighbors(point: usize) -> Vec<usize> {
    let (x, y) = FlattenedIdx::from(point);
    let mut neighbors = vec![];
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && 0 == dy {
                continue;
            }
            let final_x = x as isize + dx;
            let final_y = y as isize + dy;
            // Prevent out-of-grid neighbors
            if (final_x < 0) | (final_y < 0) | (final_x > 9) | (final_y > 9) {
                continue;
            }
            neighbors.push((final_x as usize, final_y as usize).idx())
        }
    }
    neighbors
}

#[derive(Debug)]
pub struct Grid {
    pub octopi: Octopi,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut octopi: Octopi = [0; 100];
        input.split('\n').enumerate().for_each(|(y, s)| {
            s.parse::<String>()
                .unwrap()
                .chars()
                .enumerate()
                .for_each(|(x, d)| {
                    octopi[(x, y).idx()] = d.to_digit(10).unwrap() as usize
                })
        });
        Self { octopi }
    }
}

pub struct OctopusReport {
    flashes: usize,
    zeroes: usize,
}

impl Iterator for Grid {
    type Item = OctopusReport;

    fn next(&mut self) -> Option<Self::Item> {
        let mut flashed: HashSet<usize> =
            HashSet::with_capacity(WIDTH * HEIGHT);
        let mut length = flashed.len();
        // First, the energy level of each octopus increases by 1
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                self.octopi[(x, y).idx()] += 1;
            }
        }
        loop {
            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    let idx = (x, y).idx();
                    // Then, any octopus with an energy level greater than 9 flashes
                    if self.octopi[idx] > 9 {
                        // An octopus can only flash at most once per step
                        if !flashed.insert(idx) {
                            continue;
                        }
                        // This increases the energy level of all adjacent octopuses by 1
                        for neighbor in get_neighbors(idx) {
                            self.octopi[neighbor] += 1;
                        }
                    }
                }
            }
            // This process continues as long as new octopuses keep having their
            // energy level increased beyond 9
            if flashed.len() == length {
                break;
            }
            length = flashed.len();
        }
        // Finally, any octopus that flashed during this step has its energy level set to 0
        for coord in &flashed {
            self.octopi[*coord] = 0;
        }

        Some(Self::Item {
            flashes: flashed.len(),
            zeroes: self
                .octopi
                .iter()
                .filter(|n| n == &&0usize)
                .count(),
        })
    }
}

pub fn day11a(input: &str, steps: usize) -> usize {
    Grid::new(input)
        .take(steps)
        .fold(0usize, |acc, report| acc + report.flashes)
}

pub fn day11b(input: &str) -> usize {
    Grid::new(input)
        .into_iter()
        .position(|report| report.zeroes == WIDTH * HEIGHT)
        .unwrap()
        + 1
}
