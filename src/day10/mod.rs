mod verify;
pub fn input_to_lines(input: &str) -> Vec<Vec<char>> {
    input
        .split('\n')
        .map(|l| l.chars().collect())
        .collect()
}

pub fn day10a(lines: Vec<Vec<char>>) -> usize {
    let mut sum = 0usize;
    for line in lines {
        let mut stack = Vec::with_capacity(line.len());
        for c in line {
            if ['(', '[', '{', '<'].contains(&c) {
                stack.push(c);
                continue;
            };
            if let Some(ch) = stack.pop() {
                if ch.pair() != c {
                    sum += c.value();
                    break;
                }
            }
        }
    }

    sum
}

pub fn day10b(lines: Vec<Vec<char>>) -> usize {
    let mut incomplete = Vec::new();
    for line in lines {
        let mut stack = Vec::with_capacity(line.len());
        let mut corrupted = false;
        for c in line {
            if ['(', '[', '{', '<'].contains(&c) {
                stack.push(c);
                continue;
            };
            if let Some(ch) = stack.pop() {
                if ch.pair() != c {
                    corrupted = true;
                    break;
                }
            }
        }
        if !corrupted {
            incomplete.push(stack);
        }
    }
    let mut scores: Vec<usize> = incomplete
        .iter()
        .map(|l| {
            l.iter()
                .rev()
                .fold(0usize, |acc, c| acc * 5 + c.value())
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

trait Pair {
    fn pair(&self) -> Self;
    fn value(&self) -> usize;
}

impl Pair for char {
    fn value(&self) -> usize {
        match self {
            // Part a
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            // Part b
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => 0,
        }
    }
    fn pair(&self) -> Self {
        match self {
            '(' => ')',
            '{' => '}',
            '[' => ']',
            '<' => '>',
            ')' => '(',
            '}' => '{',
            ']' => '[',
            '>' => '<',
            _ => panic!("not implemented for {}", self),
        }
    }
}
