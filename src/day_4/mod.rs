mod inputs;

pub fn run() {
    println!("---------------------");
    println!("Advent of Code Day 04");
    println!("---------------------\n");

    let result_test_first = process_input_first(inputs::test);
    let result_test_second = process_input_second(inputs::test);
    println!(
        "test:   first half: {:?}, second half: {:?}",
        result_test_first, result_test_second
    );

    let result_actual_first = process_input_first(inputs::actual);
    let result_actual_second = process_input_second(inputs::actual);
    println!(
        "actual: first half: {}, second half: {}\n",
        result_actual_first, result_actual_second
    );
}

fn process_input_first(raw: &str) -> u32 {
    raw.split("\n")
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

fn process_input_second(raw: &str) -> u32 {
    raw.split("\n")
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
