use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Sub,
};

fn main() {
    let (mut left_list, mut right_list) = get_list_from_file("input.txt");

    left_list.sort();
    right_list.sort();

    let mut total = 0;

    left_list.iter().enumerate().for_each(|(i, e)| {
        let r = right_list[i];
        let d = distance(*e, r);
        println!("distance between {} and {} = {}", e, r, d);
        total += d;
    });
    println!("total distance = {} ", total);
}

fn distance(left: i32, right: i32) -> i32 {
    left.sub(right).abs()
}

fn get_list_from_file(path: &str) -> (Vec<i32>, Vec<i32>) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    reader.lines().map_while(Result::ok).for_each(|line| {
        let splited_line: Vec<_> = line.split("   ").collect();
        left_list.push(splited_line[0].parse().unwrap());
        right_list.push(splited_line[1].parse().unwrap());
    });

    (left_list, right_list)
}
