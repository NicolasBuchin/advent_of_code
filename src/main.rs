use regex::Regex;
use std::fs::{self};

fn main() {
    let regex = Regex::new(r"mul[(]([0-9]+),([0-9]+)[)]").unwrap();

    let mul_total: i32 = fs::read_to_string("input.txt")
        .unwrap()
        .split("do()")
        .collect::<Vec<_>>()
        .iter()
        .map(|&s| s.split("don't()").collect::<Vec<&str>>()[0])
        .collect::<Vec<_>>()
        .iter()
        .map(|&s| {
            regex
                .captures_iter(s)
                .map(|c| {
                    let (_, [left, right]) = c.extract();
                    left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap()
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("result of all valid muls = {} ", mul_total);
}
