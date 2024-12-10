use advent_of_code::hoof_it;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

fn benchmark_hoof_it(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    c.bench_function("hoof_it", |b| b.iter(|| hoof_it(black_box(&input))));
}

criterion_group!(benches, benchmark_hoof_it);
criterion_main!(benches);
