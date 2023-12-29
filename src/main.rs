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
            "day5" => match args.part.as_str() {
                "part1" => days::day5::part1(&input).unwrap(),
                "part2" => days::day5::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day6" => match args.part.as_str() {
                "part1" => days::day6::part1(&input).unwrap(),
                "part2" => days::day6::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day7" => match args.part.as_str() {
                "part1" => days::day7::part1(&input).unwrap(),
                "part2" => days::day7::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day8" => match args.part.as_str() {
                "part1" => days::day8::part1(&input).unwrap(),
                "part2" => days::day8::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day9" => match args.part.as_str() {
                "part1" => days::day9::part1(&input).unwrap(),
                "part2" => days::day9::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day10" => match args.part.as_str() {
                "part1" => days::day10::part1(&input).unwrap(),
                "part2" => days::day10::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day11" => match args.part.as_str() {
                "part1" => days::day11::part1(&input).unwrap(),
                "part2" => days::day11::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day12" => match args.part.as_str() {
                "part1" => days::day12::part1(&input).unwrap(),
                "part2" => days::day12::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day13" => match args.part.as_str() {
                "part1" => days::day13::part1(&input).unwrap(),
                "part2" => days::day13::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day14" => match args.part.as_str() {
                "part1" => days::day14::part1(&input).unwrap(),
                "part2" => days::day14::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day15" => match args.part.as_str() {
                "part1" => days::day15::part1(&input).unwrap(),
                "part2" => days::day15::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day16" => match args.part.as_str() {
                "part1" => days::day16::part1(&input).unwrap(),
                "part2" => days::day16::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day17" => match args.part.as_str() {
                "part1" => days::day17::part1(&input).unwrap(),
                "part2" => days::day17::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day18" => match args.part.as_str() {
                "part1" => days::day18::part1(&input).unwrap(),
                "part2" => days::day18::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day19" => match args.part.as_str() {
                "part1" => days::day19::part1(&input).unwrap(),
                "part2" => days::day19::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day20" => match args.part.as_str() {
                "part1" => days::day20::part1(&input).unwrap(),
                "part2" => days::day20::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day21" => match args.part.as_str() {
                "part1" => days::day21::part1(&input).unwrap(),
                "part2" => days::day21::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day22" => match args.part.as_str() {
                "part1" => days::day22::part1(&input).unwrap(),
                "part2" => days::day22::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day23" => match args.part.as_str() {
                "part1" => days::day23::part1(&input).unwrap(),
                "part2" => days::day23::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            "day24" => match args.part.as_str() {
                "part1" => days::day24::part1(&input).unwrap(),
                "part2" => days::day24::part2(&input).unwrap(),
                _ => format!("Invalid part {}", args.part),
            },
            _ => format!("Invalid day {}", args.day),
        }
    )
}
