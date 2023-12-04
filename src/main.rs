extern crate clap;

use clap::Parser;

mod days;

#[macro_use]
mod util;

#[derive(Parser, Debug)]
struct CLI {
    /// The day to run
    day: String,

    /// The part to run
    part: String,
}

fn main() {
    let args = CLI::parse();

    let input = std::fs::read_to_string(format!("./inputs/{}.txt", args.day)).unwrap();

    println!(
        "{}",
        match args.day.as_str() {
            "day1" => match args.part.as_str() {
                "part1" => days::day1::part1(&input).unwrap(),
                "part2" => days::day1::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day2" => match args.part.as_str() {
                "part1" => days::day2::part1(&input).unwrap(),
                "part2" => days::day2::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day3" => match args.part.as_str() {
                "part1" => days::day3::part1(&input).unwrap(),
                "part2" => days::day3::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day4" => match args.part.as_str() {
                "part1" => days::day4::part1(&input).unwrap(),
                "part2" => days::day4::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            _ => format!("Invalid day {}", args.day),
        }
    )
}
