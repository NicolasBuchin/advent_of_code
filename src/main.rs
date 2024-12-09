use advent_of_code::disk_fragmenter;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    let solution = disk_fragmenter(&input);
    assert_eq!(solution, 6423258376982);
    println!("count: {}", solution);
}
