use advent_of_code::resonant_collinearity;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    let solution = resonant_collinearity(&input);
    assert_eq!(solution, 1169);
    println!("count: {}", solution);
}
