use rayon::{iter::ParallelIterator, str::ParallelString};

const DEPTH: usize = 2000;

pub fn monkey_market(input: &str) -> usize {
    input
        .par_lines()
        .map(|line| evaluate(line.parse().unwrap(), DEPTH))
        .sum()
}

pub fn evaluate(mut seed: u32, depth: usize) -> usize {
    for _ in 0..depth {
        seed ^= seed << 6; // * 64
        seed &= 0b111111111111111111111111;
        seed ^= seed >> 5; // / 32
        seed &= 0b111111111111111111111111;
        seed ^= seed << 11; // * 2048
        seed &= 0b111111111111111111111111;
    }
    seed as usize
}
