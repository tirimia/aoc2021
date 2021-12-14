mod verify;
use std::collections::HashMap;

type Pair = (char, char);

pub struct Polymerizer {
    pairs: HashMap<Pair, usize>,
    rules: HashMap<Pair, char>,
}

impl Polymerizer {
    fn counts(&self) -> Vec<usize> {
        let mut counts = HashMap::new();
        for (pair, count) in &self.pairs {
            *counts.entry(pair.0).or_insert(0usize) += count;
            *counts.entry(pair.1).or_insert(0usize) += count;
        }
        counts
            .values()
            .into_iter()
            .map(|v| (*v as f64 / 2.0).ceil() as usize)
            .collect()
    }
}

impl Iterator for Polymerizer {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new: HashMap<Pair, usize> = HashMap::new();
        for (point, count) in &self.pairs {
            if let Some(p) = self.rules.get(point) {
                *new.entry((point.0, *p)).or_insert(0usize) += count;
                *new.entry((*p, point.1)).or_insert(0usize) += count;
            } else {
                new.insert(*point, *count);
            }
        }
        self.pairs = new;
        let counts = self.counts();
        let max = counts.iter().max().unwrap();
        let min = counts.iter().min().unwrap();
        Some(max - min)
    }
}

impl Polymerizer {
    fn new(s: &str) -> Self {
        let (polymer_raw, rules_raw) = s.split_once("\n\n").unwrap();
        let mut counts = HashMap::new();
        for c in polymer_raw
            .chars()
            .zip(polymer_raw.chars().skip(1))
        {
            *counts.entry(c).or_insert(0usize) += 1;
        }
        let rules = rules_raw
            .split('\n')
            .filter_map(|l| l.split_once(" -> "))
            .map(|(pair, c)| {
                // Absolutely filthy, but works for now
                let mut chars = pair.chars();
                return (
                    (chars.next().unwrap(), chars.next().unwrap()),
                    c.chars().next().unwrap(),
                );
            })
            .collect();
        Self {
            pairs: counts,
            rules,
        }
    }
}

fn day14_base(puzzle: &str, days: usize) -> usize {
    if let Some(v) = Polymerizer::new(puzzle).into_iter().nth(days - 1) {
        v
    } else {
        0usize
    }
}

pub fn day14a(puzzle: &str) -> usize { day14_base(puzzle, 10) }
pub fn day14b(puzzle: &str) -> usize { day14_base(puzzle, 40) }
