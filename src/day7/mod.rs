use std::cmp::{
    max,
    min,
};

mod verify;

pub fn day7a(crabs: Vec<usize>) -> usize {
    let calculator = |min: usize, max: usize| max - min;
    day7_base(crabs, Box::new(calculator))
}

pub fn day7b(crabs: Vec<usize>) -> usize {
    let calculator = |min: usize, max: usize| (max - min) * (max - min + 1) / 2;
    day7_base(crabs, Box::new(calculator))
}

/// I pray there is a more sane/efficient way to do this, otherwise I'll be very sad
fn day7_base(
    crabs: Vec<usize>,
    calculator: Box<dyn Fn(usize, usize) -> usize>,
) -> usize {
    (0..crabs.clone().into_iter().max().unwrap())
        .into_iter()
        .map(|attempt| {
            crabs
                .iter()
                .map(|crab| {
                    calculator(min(attempt, *crab), max(attempt, *crab))
                })
                .sum()
        })
        .min()
        .unwrap()
}
