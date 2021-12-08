#![allow(clippy::many_single_char_names)]
mod verify;
use std::{
    collections::HashSet,
    iter::FromIterator,
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
            .map(|sig| sig.parse::<String>().unwrap().sort())
            .collect();
        let output: Vec<String> = parts[1]
            .trim()
            .split(' ')
            .map(|sig| sig.parse::<String>().unwrap().sort())
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

/// There is a brute force solution out there, but hell naw
pub fn day8b(readings: Vec<Reading>) -> usize {
    let mut result = 0usize;
    for reading in readings {
        let mut segments: [char; 10] = [' '; 10];
        let one = reading
            .signals
            .iter()
            .find(|r| r.len() == 2)
            .unwrap()
            .to_string();
        let four = reading
            .signals
            .iter()
            .find(|r| r.len() == 4)
            .unwrap()
            .to_string();
        let seven = reading
            .signals
            .iter()
            .find(|r| r.len() == 3)
            .unwrap()
            .to_string();
        let eight = reading
            .signals
            .iter()
            .find(|r| r.len() == 7)
            .unwrap()
            .to_string();
        let two_three_five: Vec<String> = reading
            .signals
            .iter()
            .filter(|r| r.len() == 5)
            .map(|r| r.to_owned())
            .collect();
        let zero_six_nine: Vec<String> = reading
            .signals
            .iter()
            .filter(|r| r.len() == 6)
            .map(|r| r.to_owned())
            .collect();
        let a = seven.but_not_in(&one);
        let b_d = four.but_not_in(&one);
        let e_g = eight
            .but_not_in(&four)
            .into_iter()
            .collect::<String>()
            .but_not_in(&seven)
            .into_iter()
            .collect::<String>();
        let a_d_g = two_three_five[0]
            .but_not_in(&two_three_five[1])
            .into_iter()
            .collect::<String>()
            .but_not_in(&two_three_five[2])
            .into_iter()
            .collect::<String>();
        let a_b_f_g = zero_six_nine[0]
            .but_not_in(&zero_six_nine[1])
            .into_iter()
            .collect::<String>()
            .but_not_in(&zero_six_nine[2])
            .into_iter()
            .collect::<String>();
        let g = a_d_g
            .but_not_in(&a.clone().into_iter().collect())
            .into_iter()
            .collect::<String>()
            .but_not_in(&b_d.clone().into_iter().collect())
            .into_iter()
            .collect::<String>();
        dbg!(&g);
        let e: String = e_g.but_not_in(&g).into_iter().collect();
        dbg!(&e);
        let d = a_d_g
            .but_not_in(&g)
            .into_iter()
            .collect::<String>()
            .but_not_in(&a.clone().into_iter().collect())
            .into_iter()
            .collect();
        dbg!(&d);
        let b = b_d
            .clone()
            .into_iter()
            .collect::<String>()
            .but_not_in(&d);
        dbg!(&b);
        let f = a_b_f_g
            .but_not_in(&a.clone().into_iter().collect())
            .into_iter()
            .collect::<String>()
            .but_not_in(&b.clone().into_iter().collect())
            .into_iter()
            .collect::<String>()
            .but_not_in(&g);
        dbg!(&f);
        let c = one.but_not_in(&f.clone().into_iter().collect());
        dbg!(&c);

        let indexes: [char; 7] = [
            a[0],
            b[0],
            c[0],
            d.chars().next().unwrap(),
            e.chars().next().unwrap(),
            f[0],
            g.chars().next().unwrap(),
        ];
        dbg!(indexes);
        // result += reading
        //     .output
        //     .into_iter()
        //     .map(|s| {
        //         known
        //             .iter()
        //             .position(|item| **item == String::from_str(&s).unwrap())
        //             .unwrap()
        //     })
        //     .zip([1000, 100, 10, 1usize].iter())
        //     .map(|(digit, factor)| digit * *factor)
        //     .sum::<usize>();
        // result += reading.output.iter().map(|signal| signal.as_digit(known)).zip([1000,100,10,1usize].iter()).map(|(digit, factor)| digit * *factor).sum::<usize>();
    }
    result
}

pub trait Deduce {
    fn but_not_in(&self, other: &Self) -> Vec<char>;
    fn as_digit(&self, bits: [char; 7]) -> usize;
    fn sort(&self) -> Self;
}

impl Deduce for String {
    fn sort(&self) -> Self {
        let mut chars: Vec<_> = self.chars().collect();
        chars.sort_unstable();
        chars.into_iter().collect()
    }
    fn but_not_in(&self, other: &Self) -> Vec<char> {
        self.chars()
            .filter(|c| !other.contains(*c))
            .collect()
    }

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
