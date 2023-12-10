extern crate aoc_lib;

use aoc_pm::benchmark;
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    rayon::ThreadPoolBuilder::new()
        .stack_size(100_000_000)
        .build_global()
        .unwrap();

    benchmark!(2023, 01);
    benchmark!(2023, 02);
    benchmark!(2023, 03);
    benchmark!(2023, 04);
    benchmark!(2023, 05);
    benchmark!(2023, 06);
    benchmark!(2023, 07);
    benchmark!(2023, 08);
    benchmark!(2023, 09);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
