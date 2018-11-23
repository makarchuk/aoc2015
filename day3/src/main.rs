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

struct Santa<I: Iterator<Item = char>> {
    loc: Option<Location>,
    route: I,
}

impl<I: Iterator<Item = char>> Santa<I> {
    fn new(it: I) -> Santa<I> {
        return Santa {
            loc: None,
            route: it,
        };
    }
}

impl<I: Iterator<Item = char>> Iterator for Santa<I> {
    type Item = Location;

    fn next(&mut self) -> Option<Location> {
        match self.loc {
            None => {
                self.loc = Some(Location { x: 0, y: 0 });
                self.loc
            }
            Some(loc) => match self.route.next() {
                Some(ch) => {
                    let newloc = Some(loc.step(ch));
                    self.loc = newloc;
                    self.loc.clone()
                }
                None => None,
            },
        }
    }
}

fn part1(input: &String) {
    let mut hs = HashSet::new();
    hs.extend(Santa::new(input.chars()));
    println!("Children with gifts: {}", hs.len());
}

fn part2(input: &String) {
    let mut hs = HashSet::new();
    hs.extend(Santa::new(
        input
            .char_indices()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(_, ch)| ch),
    ));
    hs.extend(Santa::new(
        input
            .char_indices()
            .filter(|(i, _)| i % 2 != 0)
            .map(|(_, ch)| ch),
    ));
    println!("Children with gifts: {}! Thanks to robot!", hs.len());
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    part1(&input);
    part2(&input);
}
