use rayon::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering};

const DEPTH: usize = 2000;

pub fn monkey_market(input: &str) -> usize {
    let scores_table = [const { AtomicI32::new(0) }; 19 * 19 * 19 * 19];

    let seeds: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();

    seeds.par_iter().for_each(|&seed| {
        evaluate_parallel(seed, DEPTH, &scores_table);
    });

    get_best_score(&scores_table)
}

fn get_best_score(scores_table: &[AtomicI32]) -> usize {
    scores_table
        .par_iter()
        .map(|score| score.load(Ordering::Relaxed))
        .max()
        .unwrap_or(0) as usize
}

fn evaluate_parallel(mut seed: i32, depth: usize, scores_table: &[AtomicI32]) {
    let mut prev_price = seed.rem_euclid(10);
    let mut sequences_todo = [true; 19 * 19 * 19 * 19];
    let mut sequence = [0, 0, 0, 0];

    for i in 0..4 {
        advance(&mut seed);
        let price = seed.rem_euclid(10);
        sequence[i] = (price - prev_price + 9) as usize;
        prev_price = price;
    }

    for _ in 4..depth {
        advance(&mut seed);
        let price = seed.rem_euclid(10);

        sequence[0] = sequence[1];
        sequence[1] = sequence[2];
        sequence[2] = sequence[3];
        sequence[3] = (price - prev_price + 9) as usize;

        let index = sequence[0] + sequence[1] * 19 + sequence[2] * 361 + sequence[3] * 6859;
        if sequences_todo[index] {
            scores_table[index].fetch_add(price, Ordering::Relaxed);
            sequences_todo[index] = false;
        }
        prev_price = price;
    }
}

fn advance(seed: &mut i32) {
    *seed ^= *seed << 6;
    *seed &= 0xFFFFFF;
    *seed ^= *seed >> 5;
    *seed &= 0xFFFFFF;
    *seed ^= *seed << 11;
    *seed &= 0xFFFFFF;
}
