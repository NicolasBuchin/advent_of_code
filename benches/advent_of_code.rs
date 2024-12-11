use advent_of_code::plutonian_pebbles;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

fn benchmark_plutonian_pebbles(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    c.bench_function("plutonian_pebbles", |b| {
        b.iter(|| plutonian_pebbles(black_box(&input)))
    });
}

criterion_group!(benches, benchmark_plutonian_pebbles);
criterion_main!(benches);
