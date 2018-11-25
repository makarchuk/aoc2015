use std::io::{self, Read};

fn three_vowels(line: &str) -> bool {
    line.chars().filter(|ch| "aeiou".contains(*ch)).count() >= 3
}

fn double_leter(line: &str) -> bool {
    //Something that never occurs in the string
    let mut last_char = '\n';
    for ch in line.chars() {
        if ch == last_char {
            return true;
        }
        last_char = ch;
    }
    return false;
}

fn blacklist(line: &str) -> bool {
    !vec!["ab", "cd", "pq", "xy"]
        .iter()
        .any(|x| line.contains(x))
}

fn non_overlaping_pairs(line: &str) -> bool {
    let mut last_char = '\n';
    let pairs: Vec<_> = line
        .chars()
        .map(|ch| match last_char {
            '\n' => {
                last_char = ch;
                String::new()
            }
            _ => {
                let mut st = String::with_capacity(2);
                st.push(last_char);
                st.push(ch);
                last_char = ch;
                st
            }
        }).filter(|line| line != "")
        .enumerate()
        .collect();
    return (&pairs).iter().any(|(i, pair)| {
        pairs
            .iter()
            .any(|p| p.0 > *i && p.1 == *pair && p.0 - i >= 2)
    });
}

fn repeated_char(line: &str) -> bool {
    let chars: Vec<char> = line.chars().collect();
    chars
        .iter()
        .enumerate()
        .any(|(i, ch)| match chars.get(i + 2) {
            Some(repeated) => repeated == ch,
            _ => false,
        })
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let good_strings = input
        .split("\n")
        .filter(|line| three_vowels(line) && double_leter(line) && blacklist(line))
        .count();
    println!("There are {} good strings!", good_strings);
    let good_strings_part_2 = input
        .split("\n")
        .filter(|line| non_overlaping_pairs(line) && repeated_char(line))
        .count();
    println!("There are {} part 2 good strings!", good_strings_part_2);
}
