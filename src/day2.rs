use std::fs;

pub fn solve() {
    println!("=== Day 2 ===");
    
    let input = read_input();
    
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);
    
    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
}

fn read_input() -> String {
    fs::read_to_string("inputs/d2p1.txt")
        .expect("Failed to read input file")
}

fn part1(input: &str) -> u64 {
    let mut addup = 0;
    for elem in input.split(',') {
        let (split_left, split_right) = elem.split_once('-').expect("should have 2 bounds");
        let lower: u64 = split_left.parse().expect("should be a number");
        let upper: u64 = split_right.parse().expect("should be a number");
        for i in lower..=upper {
            let str = i.to_string();
            if str.len() % 2 == 0 {
                let half = str.len() / 2;
                let (first_half, second_half) = str.split_at(half);
                if first_half == second_half {
                    addup += i;
                }
            }
        }
    }
    return addup;
}

fn part2(input: &str) -> u64 {
    let mut addup = 0;
    for elem in input.split(',') {
        let (lower, upper) = elem.split_once('-').expect("should have 2 bounds");
        let lower: u64 = lower.parse().expect("should be a number");
        let upper: u64 = upper.parse().expect("should be a number");
        
        'outer: for i in lower..=upper {
            let s = i.to_string();
            let len = s.len();
            
            for chunk_size in 1..=len / 2 {
                if len % chunk_size != 0 {
                    continue;
                }
                
                let first_chunk = &s[0..chunk_size];
                let all_equal = (0..len)
                    .step_by(chunk_size)
                    .all(|idx| &s[idx..idx + chunk_size] == first_chunk);
                
                if all_equal {
                    addup += i;
                    continue 'outer;
                }
            }
        }
    }
    addup
}