use std::io::{self, Read};
use std::iter::Peekable;
use std::str::CharIndices;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines: Vec<(String, usize)> = input
        .split("\n")
        .map(|line| {
            let res = parse_string(line);
            println!("Line: {} -> {}", line, res);
            (res, line.chars().count())
        }).collect();
    let part1_res = lines.iter().fold((0, 0), |summed, (ll, ol)| {
        (summed.0 + ll.chars().count(), summed.1 + ol)
    });
    println!("Part1: {}", part1_res.1 - part1_res.0);
    let part2_res = input
        .split("\n")
        .map(|line| (encode_string(line).chars().count(), line.chars().count()))
        .fold((0, 0), |sums, (el, ol)| (sums.0 + el, sums.1 + ol));
    println!("Part2: {}", part2_res.0 - part2_res.1);
}

fn parse_string(line: &str) -> String {
    let mut result = String::new();
    let mut iter = line.char_indices().peekable();
    let (_, ch) = iter.next().ok_or("Empty string").unwrap();
    if ch != '"' {
        panic!("Opening quote expected")
    }
    loop {
        let nc = next_char(&mut iter).ok_or("Unexpected end").unwrap();
        match nc {
            '"' => return result,
            '\\' => result.push_str(&read_escaped(&mut iter)),
            _ => result.push(iter.next().ok_or("Unexpected end").unwrap().1),
        }
    }
}

fn read_escaped(chars: &mut Peekable<CharIndices>) -> String {
    let mut result = String::new();
    let (_, ch) = chars.next().ok_or("Unexpected end").unwrap();
    if ch != '\\' {
        panic!("Backslash expected")
    }
    match chars.next().ok_or("Unexpected end").unwrap().1 {
        'x' => {
            let mut byte = String::with_capacity(2);
            byte.push(chars.next().ok_or("Unexpected end").unwrap().1);
            byte.push(chars.next().ok_or("Unexpected end").unwrap().1);
            result.push(u8::from_str_radix(&byte, 16).unwrap() as char);
        }
        ch => result.push(ch),
    }
    return result;
}

fn next_char(chars: &mut Peekable<CharIndices>) -> Option<char> {
    match chars.peek() {
        None => return None,
        Some(el) => {
            return Some(el.1);
        }
    }
}

fn encode_string(line: &str) -> String {
    let mut result = String::new();
    result.push('"');
    for c in line.chars() {
        match c {
            '\"' | '\\' => {
                result.push('\\');
                result.push(c);
            }
            _ => result.push(c),
        }
    }
    result.push('"');
    return result;
}
