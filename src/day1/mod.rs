mod verify;

pub fn day1a(puzzle: Vec<usize>) -> usize {
    puzzle
        .iter()
        .zip(puzzle.iter().skip(1))
        .filter(|(current, next)| current < next)
        .count()
}

pub fn day1b(puzzle: Vec<usize>) -> usize {
    day1a(
        puzzle
            .windows(3)
            .map(|w| w.iter().sum())
            .collect(),
    )
}
