pub fn read_lines(path: &str) -> Vec<String> {
    std::fs::read_to_string(&path)
        .expect("Could not load assets")
        .split('\n')
        .filter_map(|s| s.parse::<String>().ok())
        .collect()
}

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
