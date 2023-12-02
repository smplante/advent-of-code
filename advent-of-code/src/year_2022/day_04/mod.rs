mod inputs;

pub fn run() {
    let start = std::time::SystemTime::now();
    println!("--------------------------");
    println!("Advent of Code 2022 Day 04");
    println!("--------------------------\n");

    let result_test_first = process_input_first(inputs::TEST);
    let result_test_second = process_input_second(inputs::TEST);
    println!(
        "test:   first half: {:?}, second half: {:?}",
        result_test_first, result_test_second
    );

    let result_actual_first = process_input_first(inputs::ACTUAL);
    let result_actual_second = process_input_second(inputs::ACTUAL);
    println!(
        "actual: first half: {}, second half: {}\n",
        result_actual_first, result_actual_second
    );
    println!("Day 04 completed in: {:?}\n", start.elapsed().unwrap());
}

fn process_input_first(input: &str) -> u32 {
    input
        .split("\n")
        .into_iter()
        .map(|pairs| {
            pairs
                .split_once(",")
                .map(|(left, right)| {
                    let left = left
                        .split_once("-")
                        .map(|(low, high)| {
                            (
                                low.parse::<u32>().expect("values should be integers"),
                                high.parse::<u32>().expect("values should be integers"),
                            )
                        })
                        .expect("inputs should already be sanitized");
                    let right = right
                        .split_once("-")
                        .map(|(low, high)| {
                            (
                                low.parse::<u32>().expect("values should be integers"),
                                high.parse::<u32>().expect("values should be integers"),
                            )
                        })
                        .expect("inputs should already be sanitized");
                    match (left, right) {
                        ((l_low, l_high), (r_low, r_high))
                            if l_low >= r_low && l_high <= r_high =>
                        {
                            1
                        }
                        ((l_low, l_high), (r_low, r_high))
                            if r_low >= l_low && r_high <= l_high =>
                        {
                            1
                        }
                        _ => 0,
                    }
                })
                .expect("inputs should already be sanitized")
        })
        .sum()
}

fn process_input_second(input: &str) -> u32 {
    input
        .split("\n")
        .into_iter()
        .map(|pairs| {
            pairs
                .split_once(",")
                .map(|(left, right)| {
                    let left = left
                        .split_once("-")
                        .map(|(low, high)| {
                            (
                                low.parse::<u32>().expect("values should be integers"),
                                high.parse::<u32>().expect("values should be integers"),
                            )
                        })
                        .expect("inputs should already be sanitized");
                    let right = right
                        .split_once("-")
                        .map(|(low, high)| {
                            (
                                low.parse::<u32>().expect("values should be integers"),
                                high.parse::<u32>().expect("values should be integers"),
                            )
                        })
                        .expect("inputs should already be sanitized");
                    match (left, right) {
                        ((l_low, l_high), (r_low, r_high))
                            if (l_low <= r_high && l_low >= r_low)
                                || (l_high >= r_low && l_high <= r_high) =>
                        {
                            1
                        }
                        ((l_low, l_high), (r_low, r_high))
                            if (r_low <= l_high && r_low >= l_low)
                                || (r_high >= l_low && r_high <= l_high) =>
                        {
                            1
                        }
                        _ => 0,
                    }
                })
                .expect("inputs should already be sanitized")
        })
        .sum()
}

// #[cfg(test)]
// mod tests {
//     extern crate test;
//
//     use super::*;
//     use test::Bencher;
//
//     #[bench]
//     fn bench_process_input_first(b: &mut Bencher) {
//         b.iter(|| process_input_first(inputs::ACTUAL));
//     }
//
//     #[bench]
//     fn bench_process_input_second(b: &mut Bencher) {
//         b.iter(|| process_input_second(inputs::ACTUAL));
//     }
// }
