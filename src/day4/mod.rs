mod verify;

#[derive(Debug)]
pub struct Game {
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
    pub fn new(raw_input: &str) -> Self {
        let input: Vec<String> =
            raw_input.split('\n').map(String::from).collect();
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

pub fn day4a(game: &mut Game) -> usize {
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

pub fn day4b(game: Game) -> usize {
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
