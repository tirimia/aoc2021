use aoc2021::{
    day1::day_1,
    day2::day_2,
    day3::day_3,
    day4::day_4,
    day5::day_5,
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1).map(|s| s.as_str()).unwrap_or("None");
    let result = match problem {
        "day1" => day_1(),
        "day2" => day_2(),
        "day3" => day_3(),
        "day4" => day_4(),
        "day5" => day_5(),
        _ => "We haven't solved that yet".to_string(),
    };
    println!("{}", result);
}
