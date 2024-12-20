use advent_of_code::race_condition;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    let solution = race_condition(&input);
    assert_eq!(solution, 1524);
    println!("count: {}", solution);
}
