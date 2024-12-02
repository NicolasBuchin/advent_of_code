use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Sub,
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut safe_count = 0;

    reader.lines().map_while(Result::ok).for_each(|line| {
        let report: Vec<_> = line.split(' ').collect();
        let report: Vec<i32> = report.iter().map(|s| s.parse().unwrap()).collect();
        let safe = is_safe(&report);
        if safe {
            safe_count += 1;
        }
        println!("report {:?} is safe ? {}", report, safe);
    });

    println!("total amount of safe reports = {} ", safe_count);
}

fn is_safe(report: &[i32]) -> bool {
    let ascending = report[0] < report[1];

    let mut prev = report[0];
    let mut res = true;

    report[1..].iter().for_each(|&x| {
        if (ascending && (x <= prev)) || (!ascending && (x >= prev)) || (x.sub(prev).abs() > 3) {
            res = false;
        }
        prev = x;
    });

    res
}
