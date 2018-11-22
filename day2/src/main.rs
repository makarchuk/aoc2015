use std::io::{self, Read};

fn calculate_area(dim: &Vec<u32>) -> u32 {
    if dim.len() != 3 {
        panic!("Some strange dimensions!");
    }
    let (a, b, c) = (dim[0], dim[1], dim[2]);
    let areas = vec![a * b, b * c, a * c];
    let min = match areas.iter().min() {
        None => panic!("Can't find min of vector!"),
        Some(x) => x,
    };
    return min + 2 * areas.iter().fold(0, |sum, i| sum + i);
}

fn calculate_ribon_length(dim: &Vec<u32>) -> u32 {
    let max = match dim.iter().max() {
        None => panic!("Can't find max of vector"),
        Some(x) => x,
    };
    (dim.iter().fold(0, |a, b| a + b) - max) * 2 + dim.iter().fold(1, |a, b| a * b)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let boxes: Vec<Vec<u32>> = input
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|line| {
            let dimensions: Vec<u32> = line
                .split("x")
                .map(|x| -> u32 { x.parse().unwrap() })
                .collect();
            dimensions
        }).collect();
    println!(
        "Total area is {}",
        boxes
            .iter()
            .map(|x| calculate_area(x))
            .fold(0, |a, b| a + b)
    );
    println!(
        "Total ribon length is {}",
        boxes
            .iter()
            .map(|x| calculate_ribon_length(x))
            .fold(0, |a, b| a + b)
    );
}
