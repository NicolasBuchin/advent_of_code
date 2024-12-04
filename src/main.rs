use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let puzzle = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();

    puzzle.iter().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            if c == 'X' {
                check_horizontal(x, y, &puzzle);
            }
        })
    });

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    puzzle.iter().for_each(|l| println!("{:?}", l));
}

fn check_horizontal(x: usize, y: usize, puzzle: &[String]) -> bool {
    if puzzle[y].chars().nth(x + 3) != Some('S') {
        println!("ok -> {} {}", x, y);
        println!("{:?}", &puzzle[y][x..x + 4]);
    }
    if x >= 3 && puzzle[y].chars().nth(x - 3) != Some('S') {
        println!("ok <- {} {}", x, y);
    }
    true
}
