use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Mul,
};

use regex::Regex;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let regex = Regex::new(r"mul[(]([0-9]+),([0-9]+)[)]").unwrap();

    let mul_total: i32 = reader.lines().flatten().map(|line| {
        regex.captures_iter(&line).map(|c| {let (_, [left, right]) = c.extract();
            left.parse::<i32>().unwrap().mul(right.parse::<i32>().unwrap())
        }).sum::<i32>()
    }).sum();

    println!("result of all valid muls = {} ", mul_total);
}
