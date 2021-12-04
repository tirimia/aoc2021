/// Pure garbage, _maybe_ refactor at a later date
use crate::read_lines;

pub fn day_4() -> String {
    let input = read_lines("assets/day4.input");
    format!(
        "Part 1: {}\nPart 2: {}",
        day4a(&mut Game::new(input.clone())),
        day4b(Game::new(input))
    )
}

#[derive(Debug)]
struct Game {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

// TODO: see if I can use vectors of usize;5 arrays
#[derive(Debug, Clone)]
struct Board {
    rows: Vec<Vec<(usize, bool)>>,
}

impl Board {
    fn new(input_rows: Vec<String>) -> Self {
        let rows = input_rows
            .into_iter()
            .map(|line| {
                line.trim_start()
                    .split(' ')
                    .into_iter()
                    .filter(|num| !num.is_empty())
                    .map(|num| (num.parse::<usize>().unwrap(), false))
                    .collect()
            })
            .collect();
        Self { rows }
    }

    fn mark(&mut self, num: usize) -> usize {
        // TODO: see if the entry is modifiable in-place
        self.rows = self
            .rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|board_num| {
                        if board_num.0 == num {
                            (num, true)
                        } else {
                            *board_num
                        }
                    })
                    .collect()
            })
            .collect();
        if self.is_winner() {
            return num * self.sum();
        }
        0usize
    }

    fn is_winner(&self) -> bool {
        // Check rows
        self.rows
            .iter()
            .any(|row| row.iter().all(|(_, set )| *set))
	    // Check columns
            | (0..5usize)
                .into_iter()
                .any(|i| self.rows.iter().all(|row| row[i].1))
    }

    fn sum(&self) -> usize {
        self.rows
            .iter()
            .map(|row| {
                row.iter()
                    .filter_map(
                        |(val, set)| if *set { None } else { Some(val) },
                    )
                    .sum::<usize>()
            })
            .sum()
    }
}

impl Game {
    fn new(input: Vec<String>) -> Self {
        let numbers = input
            .first()
            .unwrap()
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect();
        let boards = input
            .into_iter()
            .skip(1)
            .filter(|l| !l.is_empty())
            .collect::<Vec<String>>()
            .as_slice()
            .chunks(5)
            .map(|chunk| Board::new(chunk.to_vec()))
            .collect();
        Self { numbers, boards }
    }
}

/// 31424
fn day4a(game: &mut Game) -> usize {
    for num in game.numbers.clone() {
        for idx in 0..game.boards.len() {
            let score = game.boards[idx].mark(num);
            if score != 0 {
                return score;
            }
        }
    }
    0usize
}

/// 23042
fn day4b(game: Game) -> usize {
    let mut boards = game.boards;
    let mut nums = game.numbers.iter();
    while boards.len() > 1 {
        if let Some(num) = nums.next() {
            let mut new_boards = Vec::new();
            for board in boards {
                let mut check = board.clone();
                if check.mark(*num) == 0 {
                    new_boards.push(check);
                }
            }
            boards = new_boards;
        } else {
            return 0usize;
        }
    }
    let mut final_game = Game {
        numbers: nums.cloned().take_while(|_| true).collect(),
        boards,
    };
    day4a(&mut final_game)
}

#[cfg(test)]
mod tests {
    use crate::day4::{
        day4a,
        day4b,
        Game,
    };

    const RAW_INPUT: &str = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn day4a_example() {
        let input = RAW_INPUT.split('\n').map(String::from).collect();
        assert_eq!(day4a(&mut Game::new(input)), 4512)
    }
    #[test]
    fn day4b_example() {
        let input = RAW_INPUT.split('\n').map(String::from).collect();
        assert_eq!(day4b(Game::new(input)), 1924)
    }
}
