extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;
use std::io::{self, Read};

fn main() {
    let mut input = vec![];
    io::stdin().read_to_end(&mut input).unwrap();
    let mut hash = Md5::new();
    hash.input(&input);
    let mut counter = 0;
    loop {
        let mut calculatable = hash.clone();
        calculatable.input_str(&counter.to_string());
        let out = calculatable.result_str();
        if out.starts_with("00000") {
            println!("Found matching hash {} for number {}", out, counter);
        }
        if out.starts_with("000000") {
            println!("Found matching hash {} for number {}", out, counter);
            break;
        }
        counter += 1;
    }
}
