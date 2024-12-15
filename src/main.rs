use advent_of_code::warehouse_woes;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    let solution = warehouse_woes(&input);
    assert_eq!(solution, 1550677);
    println!("count: {}", solution);
}
