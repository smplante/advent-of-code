extern crate aoc_lib;

use std::str::FromStr;

fn initialize_rayon() {
    let ignore_e_cores = 2 * (num_cpus::get() - num_cpus::get_physical());

    let num_threads = std::env::var("RAYON_NUM_THREADS")
        .ok()
        .map(|s| usize::from_str(&s).ok().unwrap_or(ignore_e_cores))
        .unwrap_or(ignore_e_cores);

    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();
}

fn main() {
    initialize_rayon();
    aoc_lib::year_2022::run();
    aoc_lib::year_2023::run();
}
