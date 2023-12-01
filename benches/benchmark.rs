extern crate aoc_lib;

use std::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    rayon::ThreadPoolBuilder::new()
        .stack_size(100_000_000)
        .num_threads(6)
        .build_global().unwrap();

    let d = aoc_lib::year_2023::Data::get("day_01_part_1").expect("file to exist").data;
    let data = std::str::from_utf8(&d).expect("to be a string");

    let mut group = c.benchmark_group("2023");
    group.significance_level(0.1).sample_size(1000).measurement_time(Duration::from_secs(30));


    group.bench_function("day_01_p1", |b| b.iter(|| aoc_lib::year_2023::day_01::part_1(black_box(data))));
    // group.bench_function("day_01_p1_rayon", |b| b.iter(|| aoc_lib::year_2023::day_01::part_1_rayon(black_box(data))));
    group.bench_function("day_01_p2", |b| b.iter(|| aoc_lib::year_2023::day_01::part_2(black_box(data))));
    // group.bench_function("day_01_p2_rayon", |b| b.iter(|| aoc_lib::year_2023::day_01::part_2_rayon(black_box(data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
