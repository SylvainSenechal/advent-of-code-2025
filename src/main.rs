mod day1;

use std::env;

#[derive(Debug)]
enum Day {
    Day1
}

impl From<&str> for Day {
    fn from(s: &str) -> Self {
        match s {
            "day1" => Day::Day1,
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
        Day::Day1 => day1::solve(),
    }
}
