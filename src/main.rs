use advent_of_code::bridge_repair;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let solution = bridge_repair(&input);
    println!("count: {}", solution);
}
