use advent_of_code::claw_contraption;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

fn benchmark_claw_contraption(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    c.bench_function("claw_contraption", |b| b.iter(|| claw_contraption(black_box(&input))));
}

criterion_group!(benches, benchmark_claw_contraption);
criterion_main!(benches);
