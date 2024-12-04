use std::fs::{self};
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let t = 100_000;
    let mut avg = 0;

    (0..t).for_each(|_| {
        let now = Instant::now();

        let count = count_x(&input);

        avg += now.elapsed().as_nanos();
        assert_eq!(count, 1831);
    });

    avg /= t;

    println!("avg elapsed time: {:.2?}ns", avg);
}

fn count_x(input: &str) -> usize {
    let puzzle = input.as_bytes();
    let line_length = input.lines().next().unwrap().len() + 1;

    puzzle[line_length..puzzle.len() - line_length]
        .iter()
        .enumerate()
        .filter(|(i, &c)| {
            c == b'A'
                && (i.rem_euclid(line_length) != 0 || i.rem_euclid(line_length) != 1)
                && check_x(*i + line_length, puzzle, line_length)
        })
        .count()
}

fn check_x(i: usize, puzzle: &[u8], line_length: usize) -> bool {
    i + line_length + 1 < puzzle.len()
        && i > line_length + 1
        && puzzle[i + line_length - 1] + puzzle[i - line_length + 1] == 160
        && puzzle[i + line_length + 1] + puzzle[i - line_length - 1] == 160
}
