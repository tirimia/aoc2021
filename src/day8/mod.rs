mod verify;
use std::str::FromStr;

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
        let mut known = vec!["".to_string(); 10];
        known[1] = reading
            .signals
            .iter()
            .find(|r| r.len() == 2)
            .unwrap()
            .to_string();
        known[4] = reading
            .signals
            .iter()
            .find(|r| r.len() == 4)
            .unwrap()
            .to_string();
        known[7] = reading
            .signals
            .iter()
            .find(|r| r.len() == 3)
            .unwrap()
            .to_string();
        known[8] = reading
            .signals
            .iter()
            .find(|r| r.len() == 7)
            .unwrap()
            .to_string();
        known[6] = reading
            .signals
            .iter()
            .filter(|s| s.len() == 6)
            .filter(|s| {
                s.chars()
                    .into_iter()
                    .any(|c| !known[1].contains(c))
                    && !s
                        .chars()
                        .into_iter()
                        .all(|c| !known[1].contains(c))
            })
            .find(|_| true)
            .unwrap()
            .to_string();
        dbg!(&known);
        dbg!(&reading.signals);
        known[0] = reading
            .signals
            .iter()
            .filter(|s| s.len() == 6)
            .filter(|s| {
                s.chars()
                    .into_iter()
                    .any(|c| !known[4].contains(c))
            })
            .filter(|s| !known.contains(s))
            .find(|_| true)
            .unwrap()
            .to_string();
        // .collect::<Vec<String>>()[0];
        known[9] = reading
            .signals
            .iter()
            .filter(|s| s.len() == 6)
            .filter(|s| !known.contains(s))
            .find(|_| true)
            .unwrap()
            .to_string();
        // .collect::<Vec<String>>()[0];
        known[5] = reading
            .signals
            .iter()
            .filter(|s| s.len() == 5)
            .filter(|s| {
                s.chars()
                    .into_iter()
                    .all(|c| known[6].contains(c))
            })
            .find(|_| true)
            .unwrap()
            .to_string();
        dbg!(&known);
        dbg!(&reading.signals);
        known[3] = reading
            .signals
            .iter()
            .filter(|s| s.len() == 5)
            .filter(|s| {
                s.chars()
                    .into_iter()
                    .all(|c| known[9].contains(c))
            })
            .filter(|s| !known.contains(s))
            .find(|_| true)
            .unwrap()
            .to_string();
        known[2] = reading
            .signals
            .iter()
            .filter(|s| s.len() == 5)
            .filter(|s| !known.contains(s))
            .find(|_| true)
            .unwrap()
            .to_string();
        dbg!(&known);
        dbg!(&reading.signals);
        result += reading
            .output
            .into_iter()
            .map(|s| {
                known
                    .iter()
                    .position(|item| **item == String::from_str(&s).unwrap())
                    .unwrap()
            })
            .zip([1000, 100, 10, 1usize].iter())
            .map(|(digit, factor)| digit * *factor)
            .sum::<usize>();
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
