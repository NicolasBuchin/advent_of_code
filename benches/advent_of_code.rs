use advent_of_code::guard_gallivant;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

fn benchmark_guard_gallivant(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").unwrap();
    c.bench_function("guard_gallivant", |b| b.iter(|| guard_gallivant(black_box(&input))));
}

criterion_group!(benches, benchmark_guard_gallivant);
criterion_main!(benches);
