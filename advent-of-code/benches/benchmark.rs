extern crate aoc_lib;

use aoc_pm::benchmark;
use criterion::{criterion_group, Criterion};

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
    benchmark!(2023, 10);
    benchmark!(2023, 11);
    benchmark!(2023, 12);
    benchmark!(2023, 13);
    benchmark!(2023, 14);
    benchmark!(2023, 15);
    benchmark!(2023, 16);
    // benchmark!(2023, 17);
    benchmark!(2023, 18);
    benchmark!(2023, 19);
    benchmark!(2023, 20);
}

criterion_group!(benches, criterion_benchmark);

fn main() {
    benches();
    Criterion::default().configure_from_args().final_summary();
}
