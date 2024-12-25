use advent_of_code::keypad_conundrum;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

fn benchmark(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    c.bench_function("keypad_conundrum", |b| b.iter(|| keypad_conundrum(black_box(&input))));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
