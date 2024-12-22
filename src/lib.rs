use std::{
    collections::{HashMap, HashSet},
    ops::AddAssign,
};

const DEPTH: usize = 2000;

pub fn monkey_market(input: &str) -> usize {
    let mut scores_map = HashMap::new();
    for line in input.lines() {
        evaluate(&mut line.parse().unwrap(), DEPTH, &mut scores_map);
    }
    get_best_score(&scores_map)
}

fn get_best_score(scores_map: &HashMap<[i32; 4], i32>) -> usize {
    let mut best_score = 0;
    for &score in scores_map.values() {
        if score > best_score {
            best_score = score;
        }
    }
    best_score as usize
}

fn evaluate(seed: &mut i32, depth: usize, scores_map: &mut HashMap<[i32; 4], i32>) {
    let mut prev_price = seed.rem_euclid(10);
    let mut sequences_done = HashSet::new();
    let mut sequence = [0, 0, 0, 0];

    for i in 0..4 {
        advance(seed);
        let price = seed.rem_euclid(10);
        sequence[i] = price - prev_price;
        prev_price = price;
    }

    for _ in 4..depth {
        advance(seed);
        let price = seed.rem_euclid(10);
        update_sequence(price - prev_price, &mut sequence);
        if !sequences_done.contains(&sequence) {
            scores_map.entry(sequence).or_default().add_assign(price);
            sequences_done.insert(sequence);
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

fn update_sequence(score: i32, sequence: &mut [i32; 4]) {
    sequence[0] = sequence[1];
    sequence[1] = sequence[2];
    sequence[2] = sequence[3];
    sequence[3] = score;
}
