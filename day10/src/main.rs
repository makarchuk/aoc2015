use std::io::{self, Read};

fn transform(st: &str) -> String {
    let mut last_char = '\n';
    let mut counter = 0;
    let mut result = String::new();
    for ch in st.chars() {
        if ch == last_char {
            counter += 1
        } else {
            if last_char != '\n' {
                result.push_str(&mut counter.to_string());
                result.push(last_char);
            }
            last_char = ch;
            counter = 1;
        }
    }
    result.push_str(&mut counter.to_string());
    result.push(last_char);
    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!(
        "Len for 50 is {}",
        (1..=50)
            .fold(input, |acc, i| {
                if i == 41 {
                    println!("Len for 40 is {}", acc.chars().count())
                }
                transform(&acc)
            }).chars()
            .count()
    )
}
