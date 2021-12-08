mod verify;
use std::{
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

/// Representation of bit array
/// |---+---+---|
/// |   | 0 |   |
/// | 5 |   | 1 |
/// | 5 |   | 1 |
/// |   | 6 |   |
/// | 4 |   | 2 |
/// | 4 |   | 2 |
/// |   | 3 |   |
/// |---+---+---|

pub fn day8b(readings: Vec<Reading>) -> usize {
    let mut result = 0usize;
    for reading in readings {
        let mut known_chars: [char; 7] = [' '; 7];
        let one = reading
            .signals
            .iter()
            .find(|r| r.len() == 2)
            .unwrap();
        let four = reading
            .signals
            .iter()
            .find(|r| r.len() == 4)
            .unwrap();
        let seven = reading
            .signals
            .iter()
            .find(|r| r.len() == 3)
            .unwrap();
        let eight = reading
            .signals
            .iter()
            .find(|r| r.len() == 7)
            .unwrap();
        // Top
        known_chars[0] = seven.but_not_in(one)[0];
        // Tail of 9 - bottom
        known_chars[3] = {
            eight.but_not_in(
                &four
                    .chars()
                    .chain(seven.chars())
                    .collect::<String>(),
            )[0]
        };
        // Middle of 3
        known_chars[6] = {
	    // Looks like
	    //  ---
	    //     |
	    //     |
	    //  ---
            let capital_c_reversed = format!("{}{}", seven, known_chars[3]);
	    dbg!(&capital_c_reversed);
	    dbg!(&known_chars);
            let three = dbg!(&reading)
                .clone()
                .signals
                .into_iter()
                .filter(|r| r.len() == 5)
                .filter(|r| capital_c_reversed.chars().all(|c| r.contains(c)))
                .collect::<Vec<String>>();
            three[0].but_not_in(&capital_c_reversed)[0]
        };
        // Top left, first prong of 4
        known_chars[5] = {
            four.but_not_in(&one.clone())
                .into_iter()
                .filter(|c| c != &known_chars[6])
                .collect::<Vec<char>>()[0]
        };
        let one_and_four = 
            // 6 and 9 both take 6 segments
            // the indexes they _don't_ have in common are 1 and 4
            // if we remove duplicates, and filter out what is in number 1 (idx 1,2), we get 4
            reading
                .clone()
                .signals
                .iter()
                .filter(|s| s.len() == 6)
                .flat_map(|s| s.chars())
                .fold(String::new(), |acc, c| {
                    if acc.contains(c) {
                        acc
                    } else {
                        format!("{}{}", acc, c)
                    }
                }).but_not_in(&String::from_iter(&known_chars));
        known_chars[4] = String::from_iter(&one_and_four).but_not_in(one)[0];
        known_chars[1] = one_and_four
            .into_iter()
            .filter(|c| c != &known_chars[4])
            .collect::<Vec<char>>()[0];
        known_chars[2] =
            eight.but_not_in(&String::from_iter(known_chars))[0];
	result += reading.output.iter().map(|signal| signal.as_digit(known_chars)).zip([1000,100,10,1usize].iter()).map(|(digit, factor)| digit * *factor).sum::<usize>();
    }
    result
}

pub trait Deduce {
    fn but_not_in(&self, other: &Self) -> Vec<char>;
    fn as_digit(&self, bits: [char; 7]) -> usize;
}

impl Deduce for String {
    fn but_not_in(&self, other: &Self) -> Vec<char> {
        self.chars()
            .filter(|c| !other.contains(*c))
            .collect()
    }

    fn as_digit(&self, bits: [char; 7]) -> usize {
	let bits_set: Vec<usize> = self.chars().into_iter().map(|c| bits.iter().position(|ch| *ch == c).unwrap()).collect();
	let num = 
	    &(0..7usize)
		.into_iter()
		.map(|i| if bits_set.contains(&i)
		     {'1'} else {'0'}
		).collect::<String>();
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
	    _ => panic!("weird bits set {}", num)
	}
    }
}

