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
        .map(|&i| find_raiting(i, line_size, bytes.len(), bytes))
        .sum()
}

fn find_trails(i: usize, line_size: usize, max_length: usize, bytes: &[u8]) -> usize {
    let mut lookup = [0usize; 25];
    lookup[0] = i;
    let mut lookup_len = 1;
    let mut score = b'0';

    while score != b'9' {
        score += 1;
        let mut new_lookup = [0usize; 25];
        let mut new_lookup_len = 0;

        for i in &lookup[0..lookup_len] {
            if *i > line_size && bytes[*i - line_size] == score {
                new_lookup[new_lookup_len] = *i - line_size;
                new_lookup_len += 1;
            }
            if *i < max_length - line_size && bytes[*i + line_size] == score {
                new_lookup[new_lookup_len] = *i + line_size;
                new_lookup_len += 1;
            }
            let rem = i.rem_euclid(line_size);
            if rem > 0 && bytes[*i - 1] == score {
                new_lookup[new_lookup_len] = *i - 1;
                new_lookup_len += 1;
            }
            if rem < line_size - 1 && bytes[*i + 1] == score {
                new_lookup[new_lookup_len] = *i + 1;
                new_lookup_len += 1;
            }
        }
        lookup = new_lookup;
        lookup_len = new_lookup_len;
    }

    let mut unique = [0usize; 25];
    let mut unique_len = 0;
    for &i in &lookup[0..lookup_len] {
        let mut not_in = true;
        for &j in &unique[0..unique_len] {
            if i == j {
                not_in = false;
                break;
            }
        }
        if not_in {
            unique[unique_len] = i;
            unique_len += 1;
        }
    }
    unique_len
}

fn find_raiting(i: usize, line_size: usize, max_length: usize, bytes: &[u8]) -> usize {
    let mut lookup = [0usize; 25];
    lookup[0] = i;
    let mut lookup_len = 1;
    let mut score = b'0';

    while score != b'9' {
        score += 1;
        let mut new_lookup = [0usize; 25];
        let mut new_lookup_len = 0;

        for i in &lookup[0..lookup_len] {
            if *i > line_size && bytes[*i - line_size] == score {
                new_lookup[new_lookup_len] = *i - line_size;
                new_lookup_len += 1;
            }
            if *i < max_length - line_size && bytes[*i + line_size] == score {
                new_lookup[new_lookup_len] = *i + line_size;
                new_lookup_len += 1;
            }
            let rem = i.rem_euclid(line_size);
            if rem > 0 && bytes[*i - 1] == score {
                new_lookup[new_lookup_len] = *i - 1;
                new_lookup_len += 1;
            }
            if rem < line_size - 1 && bytes[*i + 1] == score {
                new_lookup[new_lookup_len] = *i + 1;
                new_lookup_len += 1;
            }
        }
        lookup = new_lookup;
        lookup_len = new_lookup_len;
    }

    lookup_len
}
