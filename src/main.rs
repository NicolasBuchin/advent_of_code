use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let (mut left_list, mut right_list) = get_list_from_file("input.txt");

    left_list.sort();
    right_list.sort();

    let mut sim = 0;

    left_list.iter().for_each(|e| {
        let s = similarity(*e, &right_list);
        println!("similarity for {} = {}", e, s);
        sim += s;
    });
    println!("total similarity = {} ", sim);
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

fn similarity(e: i32, right_list: &[i32]) -> i32 {
    e.wrapping_mul(right_list.iter().filter(|&x| *x == e).count() as i32)
}
