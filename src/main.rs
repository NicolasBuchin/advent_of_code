use advent_of_code::reindeer_maze;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    let solution = reindeer_maze(&input);
    assert_eq!(solution, 496);
    println!("count: {}", solution);
}
