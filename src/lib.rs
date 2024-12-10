use std::collections::HashSet;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn hoof_it(input: &str) -> usize {
    let bytes = input.as_bytes();

    let line_size = {
        let mut line_size = 0;
        for i in 0..bytes.len() {
            if bytes[i] == b'\n' {
                line_size = i + 1;
                break;
            }
        }
        line_size
    };

    let mut zeroes = [0usize; 200];
    let mut zeroes_len = 0;

    bytes.iter().enumerate().for_each(|(i, &b)| {
        if b == b'0' {
            zeroes[zeroes_len] = i;
            zeroes_len += 1;
        }
    });

    zeroes[0..zeroes_len]
        .par_iter()
        .map(|i| find_raiting(*i, b'0', line_size, bytes.len(), bytes))
        .sum()
}

fn find_score(i: usize, line_size: usize, max_length: usize, bytes: &[u8]) -> usize {
    let mut lookup = HashSet::new();
    lookup.insert(i);

    let mut score = b'0';

    while score != b'9' {
        score += 1;
        let mut new_lookup = HashSet::new();
        for i in lookup.iter() {
            if *i > line_size && bytes[*i - line_size] == score {
                new_lookup.insert(*i - line_size);
            }
            if *i < max_length - line_size && bytes[*i + line_size] == score {
                new_lookup.insert(*i + line_size);
            }
            let rem = i.rem_euclid(line_size);
            if rem > 0 && bytes[*i - 1] == score {
                new_lookup.insert(*i - 1);
            }
            if rem < line_size - 1 && bytes[*i + 1] == score {
                new_lookup.insert(*i + 1);
            }
        }
        lookup = new_lookup;
    }

    lookup.len()
}

fn find_raiting(i: usize, score: u8, line_size: usize, max_length: usize, bytes: &[u8]) -> usize {
    if bytes[i] != score {
        return 0;
    }
    if score == b'9' {
        return 1;
    }

    let mut raiting = 0;

    if i > line_size {
        raiting += find_raiting(i - line_size, score + 1, line_size, max_length, bytes);
    }
    if i < max_length - line_size {
        raiting += find_raiting(i + line_size, score + 1, line_size, max_length, bytes);
    }
    let rem = i.rem_euclid(line_size);
    if rem > 0 {
        raiting += find_raiting(i - 1, score + 1, line_size, max_length, bytes);
    }
    if rem < line_size - 1 {
        raiting += find_raiting(i + 1, score + 1, line_size, max_length, bytes);
    }

    raiting
}
