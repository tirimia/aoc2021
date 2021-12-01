use crate::read_lines;

pub fn day_1() -> String {
    let input: Vec<usize> = read_lines("assets/day1.input")
        .iter()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();
    format!("Part 1: {}\nPart 2: {}", day1a(input.clone()), day1b(input))
}

/// 1226
pub fn day1a(puzzle: Vec<usize>) -> usize {
    puzzle
        .iter()
        .zip(puzzle.iter().skip(1))
        .filter(|(current, next)| current < next)
        .count()
}

/// 1252
pub fn day1b(puzzle: Vec<usize>) -> usize {
    day1a(
        puzzle
            .windows(3)
            .map(|w| w.iter().sum())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use crate::day1::{
        day1a,
        day1b,
    };

    const INPUT: [usize; 10] =
        [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn day1a_example() { assert_eq!(day1a(INPUT.to_vec()), 7) }
    #[test]
    fn day1b_example() { assert_eq!(day1b(INPUT.to_vec()), 5) }
}
