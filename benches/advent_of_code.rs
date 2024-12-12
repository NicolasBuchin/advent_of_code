use advent_of_code::garden_groups;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

fn benchmark_garden_groups(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    c.bench_function("garden_groups", |b| b.iter(|| garden_groups(black_box(&input))));
}

criterion_group!(benches, benchmark_garden_groups);
criterion_main!(benches);
