enum Direction {
    R,
    L,
}

struct Command {
    direction: Direction,
    rotations: i32,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        let direction = match &s[0..1] {
            "R" => Direction::R,
            "L" => Direction::L,
            _ => panic!("Invalid direction"),
        };
        let rotations: i32 = s[1..].parse().expect("Invalid number of rotations");
        
        Command { direction, rotations }
    }
}

pub fn part1(input: &str) -> i32 {
    let mut dial = 50;
    let mut password = 0;

    for elem in input.lines() {
        let command = Command::from(elem);
        match command.direction {
            Direction::R => dial = (dial + command.rotations) % 100, 
            Direction::L => dial = (dial + 100 - command.rotations) % 100,
        }
        if dial == 0 {
            password += 1;
        }
    }
    return password;
}

pub fn part2(input: &str) -> i32 {
    let mut dial = 50;
    let mut password = 0;

    for elem in input.lines() {
        let command = Command::from(elem);
        let step = match command.direction {
            Direction::R => 1,
            Direction::L => - 1,
        };
        for _ in 0..command.rotations {
            dial = (dial + step) % 100;
            if dial == 0 {
                password += 1;
            }
        }
    }
    return password;
}