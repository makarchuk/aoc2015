use std::collections::HashSet;
use std::io::{self, Read};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn step(&self, ch: char) -> Self {
        match ch {
            '>' => Location {
                x: self.x + 1,
                y: self.y,
            },
            '<' => Location {
                x: self.x - 1,
                y: self.y,
            },
            '^' => Location {
                x: self.x,
                y: self.y + 1,
            },
            'v' => Location {
                x: self.x,
                y: self.y - 1,
            },
            _ => panic!("Invalid input {}", ch),
        }
    }
}

fn part1(input: &String) {
    let mut loc = Location { x: 0, y: 0 };
    let mut hs = HashSet::new();
    hs.insert(loc);
    for ch in input.chars() {
        loc = loc.step(ch);
        hs.insert(loc);
    }
    println!("Children with gifts: {}", hs.len());
}

fn part2(input: &String) {
    let mut loc = Location { x: 0, y: 0 };
    let mut robot_loc = Location { x: 0, y: 0 };
    let mut hs = HashSet::new();
    hs.insert(loc);
    for (i, ch) in input.char_indices() {
        if i % 2 == 0 {
            loc = loc.step(ch);
            hs.insert(loc);
        } else {
            robot_loc = robot_loc.step(ch);
            hs.insert(robot_loc);
        }
    }
    println!("Children with gifts: {}! Thanks to robot!", hs.len());
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);
    part1(&input);
    part2(&input);
}
