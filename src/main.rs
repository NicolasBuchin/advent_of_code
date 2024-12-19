use advent_of_code::linen_layout;
use std::fs::{self};
mod towel_tree;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    let solution = linen_layout(&input);
    assert_eq!(solution, 336);
    println!("count: {}", solution);
}
