use advent_of_code::crossed_wires;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    let solution = crossed_wires(&input);
    assert_eq!(solution, "chv,jpj,kgj,rts,vvw,z07,z12,z26");
    println!("count: {}", solution);
}
