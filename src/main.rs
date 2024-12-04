use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let mut count = 0;

    let puzzle = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();

    puzzle.iter().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            if c == 'X' {
                if check_horizontal1(x, y, &puzzle) {
                    count += 1;
                }
                if check_horizontal2(x, y, &puzzle) {
                    count += 1;
                }
                if check_vertical1(x, y, &puzzle) {
                    count += 1;
                }
                if check_vertical2(x, y, &puzzle) {
                    count += 1;
                }
                if check_diagonal1(x, y, &puzzle) {
                    count += 1;
                }
                if check_diagonal2(x, y, &puzzle) {
                    count += 1;
                }
                if check_diagonal3(x, y, &puzzle) {
                    count += 1;
                }
                if check_diagonal4(x, y, &puzzle) {
                    count += 1;
                }
            }
        })
    });

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("count: {}", count);
}

fn check_horizontal1(x: usize, y: usize, puzzle: &[String]) -> bool {
    x + 3 < puzzle[y].len()
        && puzzle[y].chars().nth(x + 3) == Some('S')
        && puzzle[y].chars().nth(x + 2) == Some('A')
        && puzzle[y].chars().nth(x + 1) == Some('M')
}
fn check_horizontal2(x: usize, y: usize, puzzle: &[String]) -> bool {
    x >= 3
        && puzzle[y].chars().nth(x - 3) == Some('S')
        && puzzle[y].chars().nth(x - 2) == Some('A')
        && puzzle[y].chars().nth(x - 1) == Some('M')
}
fn check_vertical1(x: usize, y: usize, puzzle: &[String]) -> bool {
    y + 3 < puzzle.len()
        && puzzle[y + 3].chars().nth(x) == Some('S')
        && puzzle[y + 2].chars().nth(x) == Some('A')
        && puzzle[y + 1].chars().nth(x) == Some('M')
}
fn check_vertical2(x: usize, y: usize, puzzle: &[String]) -> bool {
    y >= 3
        && puzzle[y - 3].chars().nth(x) == Some('S')
        && puzzle[y - 2].chars().nth(x) == Some('A')
        && puzzle[y - 1].chars().nth(x) == Some('M')
}
fn check_diagonal1(x: usize, y: usize, puzzle: &[String]) -> bool {
    y + 3 < puzzle.len()
        && x + 3 < puzzle[y].len()
        && puzzle[y + 3].chars().nth(x + 3) == Some('S')
        && puzzle[y + 2].chars().nth(x + 2) == Some('A')
        && puzzle[y + 1].chars().nth(x + 1) == Some('M')
}
fn check_diagonal2(x: usize, y: usize, puzzle: &[String]) -> bool {
    y + 3 < puzzle.len()
        && x >= 3
        && puzzle[y + 3].chars().nth(x - 3) == Some('S')
        && puzzle[y + 2].chars().nth(x - 2) == Some('A')
        && puzzle[y + 1].chars().nth(x - 1) == Some('M')
}
fn check_diagonal3(x: usize, y: usize, puzzle: &[String]) -> bool {
    y >= 3
        && x + 3 < puzzle[y].len()
        && puzzle[y - 3].chars().nth(x + 3) == Some('S')
        && puzzle[y - 2].chars().nth(x + 2) == Some('A')
        && puzzle[y - 1].chars().nth(x + 1) == Some('M')
}
fn check_diagonal4(x: usize, y: usize, puzzle: &[String]) -> bool {
    y >= 3
        && x >= 3
        && puzzle[y - 3].chars().nth(x - 3) == Some('S')
        && puzzle[y - 2].chars().nth(x - 2) == Some('A')
        && puzzle[y - 1].chars().nth(x - 1) == Some('M')
}
