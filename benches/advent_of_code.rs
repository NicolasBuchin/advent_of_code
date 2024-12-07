use advent_of_code::{bridge_repair, bridge_repair_par};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

fn benchmark_bridge_repair(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").unwrap();
    c.bench_function("bridge_repair", |b| b.iter(|| bridge_repair(black_box(&input))));
    c.bench_function("bridge_repair_par", |b| b.iter(|| bridge_repair_par(black_box(&input))));
}

criterion_group!(benches, benchmark_bridge_repair);
criterion_main!(benches);
