mod verify;
use std::collections::{
    HashMap,
    HashSet,
};

type Octopi = HashMap<Coords, usize>;
type Coords = (usize, usize);

pub fn input_to_octopi(input: &str) -> Octopi {
    input
        .split('\n')
        .enumerate()
        .flat_map(|(y, s)| {
            s.parse::<String>()
                .unwrap()
                .chars()
                .enumerate()
                .map(|(x, d)| ((x, y), d.to_digit(10).unwrap() as usize))
                .collect::<Vec<(Coords, usize)>>()
        })
        .collect()
}

fn get_neighbors(point: Coords) -> Vec<Coords> {
    let mut neighbors = vec![];
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && 0 == dy {
                continue;
            }
            let final_x = point.0 as isize + dx;
            let final_y = point.1 as isize + dy;
            // Prevent out-of-grid neighbors
            if (final_x < 0) | (final_y < 0) | (final_x > 9) | (final_y > 9) {
                continue;
            }
            neighbors.push((final_x as usize, final_y as usize))
        }
    }
    neighbors
}

pub fn day11a(mut octopi: Octopi, steps: usize) -> usize {
    let mut flashes = 0usize;
    for _ in 0..steps {
        let mut flashed: HashSet<Coords> = HashSet::with_capacity(100);
        let mut length = flashed.len();
        for x in 0..10 {
            for y in 0..10 {
                *octopi.entry((x, y)).or_insert(0) += 1;
            }
        }
        loop {
            for x in 0..10 {
                for y in 0..10 {
                    if octopi.get(&(x, y)).unwrap_or(&0) > &9 {
                        if !flashed.insert((x, y)) {
                            continue;
                        }
                        let neighbors = get_neighbors((x, y));
                        for neighbor in neighbors {
                            *octopi.entry(neighbor).or_insert(0) += 1;
                        }
                    }
                }
            }
            if flashed.len() == length {
                break;
            }
            length = flashed.len();
        }
        for coord in &flashed {
            *octopi.entry(*coord).or_insert(0) = 0;
        }

        flashes += flashed.len();
    }
    flashes
}

pub fn day11b(mut octopi: Octopi) -> usize {
    for step in 1..usize::MAX {
        let mut flashed: HashSet<Coords> = HashSet::with_capacity(100);
        let mut length = flashed.len();
        for x in 0..10 {
            for y in 0..10 {
                *octopi.entry((x, y)).or_insert(0) += 1;
            }
        }
        loop {
            for x in 0..10 {
                for y in 0..10 {
                    if octopi.get(&(x, y)).unwrap_or(&0) > &9 {
                        if !flashed.insert((x, y)) {
                            continue;
                        }
                        let neighbors = get_neighbors((x, y));
                        for neighbor in neighbors {
                            *octopi.entry(neighbor).or_insert(0) += 1;
                        }
                    }
                }
            }
            if flashed.len() == length {
                break;
            }
            length = flashed.len();
        }
        for coord in &flashed {
            *octopi.entry(*coord).or_insert(0) = 0;
        }

        let mut zeroes = 0usize;
        for x in 0..10 {
            for y in 0..10 {
                if *octopi.get(&(x, y)).unwrap_or(&0) == 0usize {
                    zeroes += 1;
                }
            }
        }
        if zeroes == 100 {
            return step;
        }
    }
    0usize
}
