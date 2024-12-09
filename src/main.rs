use advent_of_code::disk_fragmenter;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("test.txt").unwrap();
    let input = input.replace('\r', "");
    let input = "12345";
    let solution = disk_fragmenter(&input);
    assert_eq!(solution, 1169);
    println!("count: {}", solution);
}
