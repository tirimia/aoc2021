use std::collections::HashMap;

pub const INPUT: [usize; 300] = [
    5, 1, 2, 1, 5, 3, 1, 1, 1, 1, 1, 2, 5, 4, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1,
    1, 2, 1, 5, 1, 1, 1, 3, 1, 1, 1, 3, 1, 1, 3, 1, 1, 4, 3, 1, 1, 4, 1, 1, 1,
    1, 2, 1, 1, 1, 5, 1, 1, 5, 1, 1, 1, 4, 4, 2, 5, 1, 1, 5, 1, 1, 2, 2, 1, 2,
    1, 1, 5, 3, 1, 2, 1, 1, 3, 1, 4, 3, 3, 1, 1, 3, 1, 5, 1, 1, 3, 1, 1, 4, 4,
    1, 1, 1, 5, 1, 1, 1, 4, 4, 1, 3, 1, 4, 1, 1, 4, 5, 1, 1, 1, 4, 3, 1, 4, 1,
    1, 4, 4, 3, 5, 1, 2, 2, 1, 2, 2, 1, 1, 1, 2, 1, 1, 1, 4, 1, 1, 3, 1, 1, 2,
    1, 4, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 5, 5, 1, 1, 1, 5, 1, 1, 1, 1, 5,
    1, 3, 2, 1, 1, 5, 2, 3, 1, 2, 2, 2, 5, 1, 1, 3, 1, 1, 1, 5, 1, 4, 1, 1, 1,
    3, 2, 1, 3, 3, 1, 3, 1, 1, 1, 1, 1, 1, 1, 2, 3, 1, 5, 1, 4, 1, 3, 5, 1, 1,
    1, 2, 2, 1, 1, 1, 1, 5, 4, 1, 1, 3, 1, 2, 4, 2, 1, 1, 3, 5, 1, 1, 1, 3, 1,
    1, 1, 5, 1, 1, 1, 1, 1, 3, 1, 1, 1, 4, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 5, 3,
    1, 2, 3, 4, 1, 1, 5, 1, 2, 4, 2, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 4, 1, 5,
];

fn day6_base(state: Vec<usize>, days: usize) -> usize {
    (0..days)
        .into_iter()
        .fold(HashMap::init(state), |s, _| s.iterate())
        .total()
}

pub fn day6a(state: Vec<usize>) -> usize { day6_base(state, 80) }

pub fn day6b(state: Vec<usize>) -> usize { day6_base(state, 256) }

trait LaternfishCounter {
    fn init(state: Vec<usize>) -> Self;
    fn iterate(&self) -> Self;
    fn total(self) -> usize;
}

impl LaternfishCounter for HashMap<usize, usize> {
    fn init(state: Vec<usize>) -> Self {
        state
            .iter()
            .fold(HashMap::new(), |mut acc, item| {
                *acc.entry(*item).or_insert(0usize) += 1;
                acc
            })
    }

    fn total(self) -> usize { self.values().sum() }

    fn iterate(&self) -> Self {
        let mut new = HashMap::with_capacity(9);
        for i in 0..8 {
            *new.entry(i).or_insert(0) += self.get(&(i + 1)).unwrap_or(&0);
        }
        let zeroes = self.get(&0).unwrap_or(&0);
        *new.entry(8).or_insert(0) += *zeroes;
        *new.entry(6).or_insert(0) += *zeroes;
        new
    }
}

#[cfg(test)]
mod tests {
    use crate::day6::{
        day6a,
        day6b,
        INPUT,
    };

    const TEST_INPUT: [usize; 5] = [3, 4, 3, 1, 2];

    #[test]
    fn day6a_naive_example() {
        fn naive_day6a(state: Vec<usize>) -> usize {
            (0..80)
                .into_iter()
                .fold(state, |s, _| {
                    s.into_iter()
                        .flat_map(|fish| {
                            if fish >= 1 {
                                [fish - 1].to_vec()
                            } else {
                                [6, 8].to_vec()
                            }
                        })
                        .collect()
                })
                .len()
        }
        assert_eq!(naive_day6a(TEST_INPUT.to_vec()), 5934)
    }
    #[test]
    fn day6a_example() { assert_eq!(day6a(TEST_INPUT.to_vec()), 5934) }
    #[test]
    fn day6a_real() { assert_eq!(day6a(INPUT.to_vec()), 383160) }
    #[test]
    fn day6b_example() { assert_eq!(day6b(TEST_INPUT.to_vec()), 26984457539) }
    #[test]
    fn day6b_real() { assert_eq!(day6b(INPUT.to_vec()), 1721148811504) }
}
