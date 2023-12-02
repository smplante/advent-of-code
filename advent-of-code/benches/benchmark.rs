extern crate aoc_lib;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    rayon::ThreadPoolBuilder::new()
        .stack_size(100_000_000)
        .num_threads(6)
        .build_global()
        .unwrap();

    let mut group = c.benchmark_group("2023");
    group.sample_size(500).warm_up_time(Duration::from_secs(5));

    let d1 = aoc_lib::year_2023::Data::get("day_01_part_1")
        .expect("file to exist")
        .data;
    let data1 = std::str::from_utf8(&d1).expect("to be a string");
    group.bench_function("day_01_p1", |b| {
        b.iter(|| aoc_lib::year_2023::day_01::part_1(black_box(data1)))
    });
    group.bench_function("day_01_p2", |b| {
        b.iter(|| aoc_lib::year_2023::day_01::part_2(black_box(data1)))
    });
    group.bench_function("day_01", |b| b.iter(|| aoc_lib::year_2023::day_01::run()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
