mod verify;
use std::{
    collections::HashMap,
    str::FromStr,
};

pub fn input_to_readings(input: &str) -> Vec<Reading> {
    input
        .split('\n')
        .filter_map(|s| s.parse::<String>().ok())
        .map(|l| Reading::from_str(&l).unwrap())
        .collect()
}

#[derive(Clone, Debug)]
pub struct Reading {
    signals: Vec<String>,
    output: Vec<String>,
}

impl FromStr for Reading {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<String> = s.split('|').map(str::to_string).collect();
        let signals: Vec<String> = parts[0]
            .trim()
            .split(' ')
            .map(|sig| sig.parse::<String>().unwrap())
            .collect();
        let output: Vec<String> = parts[1]
            .trim()
            .split(' ')
            .map(|sig| sig.parse::<String>().unwrap())
            .collect();
        Ok(Self { signals, output })
    }
}

pub fn day8a(readings: Vec<Reading>) -> usize {
    readings
        .iter()
        .map(|r| {
            r.output
                .iter()
                .filter(|o| [2, 3, 4, 7].contains(&o.len()))
                .count()
        })
        .sum()
}

pub fn day8b(readings: Vec<Reading>) -> usize {
    let mut result = 0usize;
    for reading in readings {
        let four = reading
            .signals
            .iter()
            .find(|r| r.len() == 4)
            .unwrap()
            .to_string()
            .chars()
            .collect::<Vec<char>>();
        let mut counts = HashMap::new();
        for c in reading.signals.iter().flat_map(|s| s.chars()) {
            *counts.entry(c).or_insert(0usize) += 1;
        }
        let mut mapping: [char; 7] = [' '; 7];
        mapping[2] = *counts
            .iter()
            .filter(|(_, v)| **v == 9)
            .map(|(k, _)| k)
            .next()
            .unwrap();
        mapping[4] = *counts
            .iter()
            .filter(|(_, v)| **v == 4)
            .map(|(k, _)| k)
            .next()
            .unwrap();
        mapping[5] = *counts
            .iter()
            .filter(|(_, v)| **v == 6)
            .map(|(k, _)| k)
            .next()
            .unwrap();
        let sevens = counts
            .iter()
            .filter(|(_, v)| **v == 7)
            .map(|(k, _)| *k);
        let eights = counts
            .iter()
            .filter(|(_, v)| **v == 8)
            .map(|(k, _)| *k);
        for seven in sevens {
            if four.contains(&seven) {
                mapping[6] = seven;
            } else {
                mapping[3] = seven;
            }
        }
        for eight in eights {
            if four.contains(&eight) {
                mapping[1] = eight;
            } else {
                mapping[0] = eight;
            }
        }

        result += reading
            .output
            .iter()
            .map(|signal| signal.as_digit(mapping))
            .zip([1000, 100, 10, 1usize].iter())
            .map(|(digit, factor)| digit * *factor)
            .sum::<usize>();
    }
    result
}

pub trait Digify {
    fn as_digit(&self, bits: [char; 7]) -> usize;
}

impl Digify for String {
    fn as_digit(&self, bits: [char; 7]) -> usize {
        let bits_set: Vec<usize> = self
            .chars()
            .into_iter()
            .map(|c| bits.iter().position(|ch| *ch == c).unwrap())
            .collect();
        let num = &(0..7usize)
            .into_iter()
            .map(|i| if bits_set.contains(&i) { '1' } else { '0' })
            .collect::<String>();
        match num.as_ref() {
            "1111110" => 0,
            "0110000" => 1,
            "1101101" => 2,
            "1111001" => 3,
            "0110011" => 4,
            "1011011" => 5,
            "1011111" => 6,
            "1110000" => 7,
            "1111111" => 8,
            "1111011" => 9,
            _ => panic!("weird bits set {}", num),
        }
    }
}
