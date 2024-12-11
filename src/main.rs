use advent_of_code::plutonian_pebbles;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    let solution = plutonian_pebbles(&input);
    assert_eq!(solution, 225253278506288);
    println!("count: {}", solution);
}
