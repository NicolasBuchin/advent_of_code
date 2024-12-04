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
            if c == 'A' && check_x(x, y, &puzzle) {
                count += 1;
            }
        })
    });

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("count: {}", count);
}

fn check_x(x: usize, y: usize, puzzle: &[String]) -> bool {
    x >= 1
        && x + 1 < puzzle[y].len()
        && y >= 1
        && y + 1 < puzzle.len()
        && ((puzzle[y + 1].chars().nth(x + 1) == Some('S')
            && puzzle[y - 1].chars().nth(x - 1) == Some('M'))
            || (puzzle[y + 1].chars().nth(x + 1) == Some('M')
                && puzzle[y - 1].chars().nth(x - 1) == Some('S')))
        && ((puzzle[y - 1].chars().nth(x + 1) == Some('S')
            && puzzle[y + 1].chars().nth(x - 1) == Some('M'))
            || (puzzle[y - 1].chars().nth(x + 1) == Some('M')
                && puzzle[y + 1].chars().nth(x - 1) == Some('S')))
}
