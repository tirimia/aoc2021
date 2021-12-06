mod verify;
use std::collections::HashMap;

fn day6_base(state: Vec<usize>, days: usize) -> usize {
    (0..days)
        .into_iter()
        .fold(HashMap::init(state), |s, _| s.iterate())
        .total()
}

pub fn day6a(state: Vec<usize>) -> usize { day6_base(state, 80) }

pub fn day6b(state: Vec<usize>) -> usize { day6_base(state, 256) }

trait LanternfishCounter {
    fn init(state: Vec<usize>) -> Self;
    fn iterate(&self) -> Self;
    fn total(self) -> usize;
}

impl LanternfishCounter for HashMap<usize, usize> {
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
