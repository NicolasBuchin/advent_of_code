use advent_of_code::keypad_conundrum;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    let solution = keypad_conundrum(&input);
    assert_eq!(solution, 223285811665866);
    println!("count: {}", solution);
}
