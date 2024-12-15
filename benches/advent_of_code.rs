use advent_of_code::warehouse_woes;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

fn benchmark_warehouse_woes(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    c.bench_function("warehouse_woes", |b| b.iter(|| warehouse_woes(black_box(&input))));
}

criterion_group!(benches, benchmark_warehouse_woes);
criterion_main!(benches);
