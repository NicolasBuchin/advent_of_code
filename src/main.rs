use advent_of_code::monkey_market;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    let solution = monkey_market(&input);
    assert_eq!(solution, 1925);
    println!("count: {}", solution);
}
