mod verify;
use std::collections::HashMap;

const FISH_TIMERS: usize = 9;

fn day6_base(state: Vec<usize>, days: usize) -> usize {
    (0..days)
        .into_iter()
        .fold(init(state), |s, _| s.iterate())
        .iter()
        .sum()
}

pub fn day6a(state: Vec<usize>) -> usize { day6_base(state, 80) }

pub fn day6b(state: Vec<usize>) -> usize { day6_base(state, 256) }

fn init(state: Vec<usize>) -> [usize; FISH_TIMERS] {
    let mut list: [usize; FISH_TIMERS] = [0; FISH_TIMERS];
    for fish in state {
        list[fish] += 1;
    }
    list
}

trait LanternfishCounter {
    fn init(state: Vec<usize>) -> Self;
    fn iterate(&self) -> Self;
}

impl LanternfishCounter for [usize; FISH_TIMERS] {
    fn init(state: Vec<usize>) -> Self { init(state) }

    fn iterate(&self) -> Self {
        let mut list: [usize; FISH_TIMERS] = [0; FISH_TIMERS];
        list[8] = self[0];
        list[6] = self[0];
        for i in 1..FISH_TIMERS {
            list[i - 1] += self[i];
        }
        list
    }
}

/// HashMap implementation
fn day6_base_hash(state: Vec<usize>, days: usize) -> usize {
    (0..days)
        .into_iter()
        .fold(HashMap::init(state), |s, _| s.iterate())
        .values()
        .sum()
}

pub fn day6a_hash(state: Vec<usize>) -> usize { day6_base_hash(state, 80) }

pub fn day6b_hash(state: Vec<usize>) -> usize { day6_base_hash(state, 256) }

impl LanternfishCounter for HashMap<usize, usize> {
    fn init(state: Vec<usize>) -> Self {
        state
            .iter()
            .fold(HashMap::new(), |mut acc, item| {
                *acc.entry(*item).or_insert(0usize) += 1;
                acc
            })
    }

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
