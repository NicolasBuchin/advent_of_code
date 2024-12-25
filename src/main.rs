use advent_of_code::code_chronicle;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    let solution = code_chronicle(&input);
    assert_eq!(solution, 3483);
    println!("count: {}", solution);
}
