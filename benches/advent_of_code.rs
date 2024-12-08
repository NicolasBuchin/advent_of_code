use advent_of_code::resonant_collinearity;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

fn benchmark_resonant_collinearity(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    c.bench_function("resonant_collinearity", |b| {
        b.iter(|| resonant_collinearity(black_box(&input)))
    });
}

criterion_group!(benches, benchmark_resonant_collinearity);
criterion_main!(benches);
