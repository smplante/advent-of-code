extern crate aoc_lib;

use aoc_pm::benchmark;
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    rayon::ThreadPoolBuilder::new()
        .stack_size(100_000_000)
        .num_threads(6)
        .build_global()
        .unwrap();

    let mut group = c.benchmark_group("2023");
    group.sample_size(100).warm_up_time(Duration::from_secs(5));

    benchmark!(group, 2023, 01);
    benchmark!(group, 2023, 02);
    benchmark!(group, 2023, 03);
    benchmark!(group, 2023, 04);
    benchmark!(group, 2023, 05);
    benchmark!(group, 2023, 06);
    benchmark!(group, 2023, 07);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
