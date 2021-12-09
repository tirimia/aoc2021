mod verify;
use std::collections::{
    HashMap,
    HashSet,
};

type HeightMap = HashMap<(usize, usize), usize>;
type Coords = (usize, usize);
type Points = Vec<(Coords, usize)>;

pub fn input_to_coords(input: &str) -> HeightMap {
    let mut map = HashMap::new();
    let parsed_input: Vec<Vec<usize>> = input
        .split('\n')
        .filter_map(|s| s.parse::<String>().ok())
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    for (idx, line) in parsed_input.iter().enumerate() {
        for (line_idx, num) in line.iter().enumerate() {
            map.insert((line_idx, idx), *num);
        }
    }
    map
}

fn get_neighbors(point: Coords) -> Vec<Coords> {
    let mut neighbors = vec![];
    for dx in -1..=1 {
        for dy in -1..=1 {
            if (dx == 0 && 0 == dy) | (dx == dy) | (dx == -dy) {
                continue;
            }
            let final_x = point.0 as isize + dx;
            let final_y = point.1 as isize + dy;
            if (final_x < 0) | (final_y < 0) {
                continue;
            }
            neighbors.push((final_x as usize, final_y as usize))
        }
    }
    neighbors
}

fn get_lowest_points(heightmap: HeightMap) -> Points {
    heightmap
        .iter()
        .filter(|(point, val)| {
            get_neighbors(**point)
                .into_iter()
                .filter_map(|p| heightmap.get(&p))
                .all(|v| v > val)
        })
        .map(|(c, v)| (*c, *v))
        .collect()
}

fn get_non_walls_around(point: Coords, heightmap: &HeightMap) -> Vec<Coords> {
    get_neighbors(point)
        .into_iter()
        .filter(|p| *heightmap.get(p).unwrap_or(&9usize) != 9)
        .collect()
}

pub fn day9a(heightmap: HeightMap) -> usize {
    get_lowest_points(heightmap)
        .iter()
        .map(|(_, v)| v + 1)
        .sum()
}

pub fn day9b(heightmap: HeightMap) -> usize {
    let mut basins: Vec<Vec<Coords>> = vec![];
    let seeds: Vec<Coords> = get_lowest_points(heightmap.clone())
        .into_iter()
        .map(|(c, _)| c)
        .collect();
    for lowest in seeds {
        let mut current_basin = HashSet::new();
        current_basin.insert(lowest);
        let mut last_size = current_basin.len();
        loop {
            for coord in &current_basin.clone() {
                for neighbor in get_non_walls_around(*coord, &heightmap) {
                    current_basin.insert(neighbor);
                }
            }
            let current_length = current_basin.len();
            if last_size == current_length {
                break;
            } else {
                last_size = current_length;
            }
        }
        basins.push(current_basin.into_iter().collect());
    }
    let mut sizes: Vec<usize> = basins.iter().map(|b| b.len()).collect();
    sizes.sort_unstable();
    sizes.iter().rev().take(3).product()
}
