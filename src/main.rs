use advent_of_code::lan_party;
use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    let solution = lan_party(&input);
    assert_eq!(solution, "di,gs,jw,kz,md,nc,qp,rp,sa,ss,uk,xk,yn");
    println!("count: {}", solution);
}
