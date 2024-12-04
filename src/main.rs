use std::fs::{self};
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let now = Instant::now();

    let puzzle = input.as_bytes();
    let line_length = input.lines().next().unwrap().len() + 1;

    let count = puzzle
        .iter()
        .enumerate()
        .filter(|(i, &c)| c == b'A' && check_x(*i, puzzle, line_length))
        .count();

    println!("Elapsed: {:.2?}", now.elapsed());

    println!("ok: {}", count == 1831);
}

fn check_x(i: usize, puzzle: &[u8], line_length: usize) -> bool {
    i + line_length + 1 < puzzle.len()
        && i > line_length + 1
        && ((puzzle[i + line_length + 1] == b'S' && puzzle[i - line_length - 1] == b'M')
            || (puzzle[i + line_length + 1] == b'M' && puzzle[i - line_length - 1] == b'S'))
        && ((puzzle[i + line_length - 1] == b'S' && puzzle[i - line_length + 1] == b'M')
            || (puzzle[i + line_length - 1] == b'M' && puzzle[i - line_length + 1] == b'S'))
}
