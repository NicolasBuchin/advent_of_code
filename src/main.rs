use advent_of_code::garden_groups;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    let solution = garden_groups(&input);
    assert_eq!(solution, 1930);
    println!("count: {}", solution);
}
