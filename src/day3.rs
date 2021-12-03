use crate::read_lines;

pub fn day_3() -> String {
    let input: Vec<Vec<u32>> = read_lines("assets/day3.input")
        .iter()
        .map(|l| {
            l.chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    format!("Part 1: {}\nPart 2: {}", day3a(input.clone()), day3b(input))
}

fn get_counts(nums: &[Vec<u32>], idx: usize) -> [usize; 2] {
    let zeroes = nums
        .iter()
        .filter(|&s| s.get(idx).unwrap() == &0u32)
        .count();
    [zeroes, nums.len() - zeroes]
}

fn as_chars(ints: &[u32]) -> Vec<char> {
    ints.iter()
        .map(|i| char::from_digit(*i, 10).unwrap())
        .collect()
}

fn as_int(chars: &[char]) -> usize {
    usize::from_str_radix(&chars.iter().collect::<String>(), 2).unwrap()
}

/// 4160394
fn day3a(numbers: Vec<Vec<u32>>) -> usize {
    let mut gamma = vec![];
    let mut epsilon = vec![];
    for (bit, _) in numbers[0].iter().enumerate() {
        let counts = get_counts(&numbers, bit);
        if counts[0] > counts[1] {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }
    as_int(&gamma) * as_int(&epsilon)
}

trait Whittle {
    fn keep(&mut self, c: u32, at: usize);
    fn whittle(&mut self, desired_bit: u32);
}

impl Whittle for Vec<Vec<u32>> {
    fn keep(&mut self, digit: u32, at: usize) {
        self.retain(|n| n.get(at).unwrap() == &digit);
    }
    fn whittle(&mut self, desired_bit: u32) {
        for bit in 0..self[0].len() {
            if self.len() == 1 {
                break;
            }
            let counts = get_counts(self, bit);
            let reverse = desired_bit ^ 1;
            if counts[1] >= counts[0] {
                self.keep(desired_bit, bit);
            } else {
                self.keep(reverse, bit);
            }
        }
    }
}

/// 4125600
fn day3b(numbers: Vec<Vec<u32>>) -> usize {
    let mut oxygen = numbers.clone();
    let mut see_oh_two = numbers;
    oxygen.whittle(1);
    see_oh_two.whittle(0);
    as_int(&as_chars(&oxygen[0])) * as_int(&as_chars(&see_oh_two[0]))
}

#[cfg(test)]
mod tests {
    use crate::day3::{
        day3a,
        day3b,
    };

    fn get_input() -> Vec<Vec<u32>> {
        [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111",
            "11100", "10000", "11001", "00010", "01010",
        ]
        .iter()
        .map(|l| {
            l.chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect()
    }

    #[test]
    fn day3a_example() { assert_eq!(day3a(get_input()), 198) }
    #[test]
    fn day3b_example() { assert_eq!(day3b(get_input()), 230) }
}
