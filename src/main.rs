use advent_of_code::ram_run;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    let solution = ram_run(&input);
    assert_eq!(solution, "52,5");
    println!("count: {}", solution);
}
