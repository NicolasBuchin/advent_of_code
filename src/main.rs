mod matrix;
use advent_of_code::guard_gallivant;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let solution = guard_gallivant(&input);
    println!("count: {}", solution);
}
