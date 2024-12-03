use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Mul,
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mul_total: i32 = reader.lines().flatten().map(|line| {
        let indices: Vec<usize> = line.match_indices("mul").map(|(i,_)| i).collect();
        let s: Vec<char> = line.chars().collect();
        
        indices.iter().filter_map(|&index| {
            get_mul(index, &s)
        }).sum::<i32>()
    }).sum();

    println!("result of all valid muls = {} ", mul_total);
}

fn get_mul(index: usize, s: &[char]) -> Option<i32> {
    let mut i = index + 3;
    
    if s[i] != '(' {
        return None;
    }

    i += 1;
    let mut left = Vec::new();
    while s[i].is_digit(10) {
        left.push(s[i]);
        i += 1;
    }

    if s[i] != ',' {
        return None;
    }

    i += 1;
    let mut right = Vec::new();
    while s[i].is_digit(10) {
        right.push(s[i]);
        i += 1;
    }

    if s[i] != ')' {
        return None;
    }

    let x = left.into_iter().collect::<String>().parse::<i32>().unwrap();
    let y = right.into_iter().collect::<String>().parse::<i32>().unwrap();
    Some(x.mul(y))
}
