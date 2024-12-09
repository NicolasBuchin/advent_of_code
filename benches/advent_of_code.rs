use advent_of_code::disk_fragmenter;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

fn benchmark_disk_fragmenter(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.replace('\r', "");
    c.bench_function("disk_fragmenter", |b| {
        b.iter(|| disk_fragmenter(black_box(&input)))
    });
}

criterion_group!(benches, benchmark_disk_fragmenter);
criterion_main!(benches);
