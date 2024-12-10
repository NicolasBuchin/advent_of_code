use advent_of_code::hoof_it;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    let solution = hoof_it(&input);
    assert_eq!(solution, 966);
    println!("count: {}", solution);
}
