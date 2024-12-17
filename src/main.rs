use advent_of_code::chronospatial_computer;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    let solution = chronospatial_computer(&input);
    assert_eq!(solution, 496);
    println!("count: {}", solution);
}
