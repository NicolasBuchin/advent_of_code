use advent_of_code::crossed_wires;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    let solution = crossed_wires(&input);
    assert_eq!(solution, 56620966442854);
    println!("count: {}", solution);
}
