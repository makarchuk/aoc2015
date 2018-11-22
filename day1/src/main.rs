use std::io::{self, Read};

fn main() {
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap();
    let mut counter: i32 = 0;
    let mut first_basement_entrance = 0;
    for (i, ch) in data.chars().enumerate() {
        match ch {
            '(' => counter += 1,
            ')' => {
                counter -= 1;
                if counter < 0 && first_basement_entrance == 0 {
                    first_basement_entrance = i + 1;
                }
            }
            _ => panic!("I don't know what this charachter means"),
        }
    }
    println!("Resulting floor is {}", counter);
    println!(
        "First entered a basement at position {}",
        first_basement_entrance
    );
}
