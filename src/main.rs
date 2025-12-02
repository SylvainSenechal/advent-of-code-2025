mod day1;
mod day2;

use std::env;
use std::fmt::Display;
use std::fs;

pub fn read_input(day: u8) -> String {
    let filename = format!("inputs/d{}p1.txt", day);
    fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", filename))
}

pub fn solve<P1, P2>(day: u8, part1_fn: fn(&str) -> P1, part2_fn: fn(&str) -> P2)
where
    P1: Display,
    P2: Display,
{
    println!("=== Day {} ===", day);
    let input = read_input(day);
    
    let part1_result = part1_fn(&input);
    println!("Part 1: {}", part1_result);
    
    let part2_result = part2_fn(&input);
    println!("Part 2: {}", part2_result);
}

#[derive(Debug)]
enum Day {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
}

impl From<&str> for Day {
    fn from(s: &str) -> Self {
        match s {
            "day1" => Day::Day1,
            "day2" => Day::Day2,
            "day3" => Day::Day3,
            "day4" => Day::Day4,
            "day5" => Day::Day5,
            "day6" => Day::Day6,
            "day7" => Day::Day7,
            "day8" => Day::Day8,
            "day9" => Day::Day9,
            "day10" => Day::Day10,
            "day11" => Day::Day11,
            "day12" => Day::Day12,
            _ => {
                eprintln!("Unknown day: {}", s);
                std::process::exit(1);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        eprintln!("Example: {} day1", args[0]);
        std::process::exit(1);
    }
    
    let day: Day = args[1].as_str().into();
    
    match day {
        Day::Day1 => solve(1, day1::part1, day1::part2),
        Day::Day2 => solve(2, day2::part1, day2::part2),
        Day::Day3 => unimplemented!(),
        Day::Day4 => unimplemented!(),
        Day::Day5 => unimplemented!(),
        Day::Day6 => unimplemented!(),
        Day::Day7 => unimplemented!(),
        Day::Day8 => unimplemented!(),
        Day::Day9 => unimplemented!(),
        Day::Day10 => unimplemented!(),
        Day::Day11 => unimplemented!(),
        Day::Day12 => unimplemented!(),
    }
}
