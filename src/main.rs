use advent_of_code::bridge_repair_par;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let solution = bridge_repair_par(&input);
    assert_eq!(solution, 254136560217241);
    println!("count: {}", solution);
}
