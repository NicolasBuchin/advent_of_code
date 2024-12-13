use advent_of_code::claw_contraption;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    let solution = claw_contraption(&input);
    assert_eq!(solution, 90798500745591);
    println!("count: {}", solution);
}
