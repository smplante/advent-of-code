use std::collections::HashSet;

mod inputs;

pub fn run() {
    let start = std::time::SystemTime::now();
    println!("--------------------------");
    println!("Advent of Code 2022 Day 06");
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
        "actual: first half: {:?}, second half: {:?}\n",
        result_actual_first, result_actual_second
    );
    println!("Day 06 completed in: {:?}\n", start.elapsed().unwrap());
}

fn process_input_first(input: &str) -> u32 {
    let size = 4;
    let mut pos = 0;
    let chars = input.chars().into_iter().collect::<Vec<char>>();
    let windows = chars.windows(size);
    for window in windows {
        if window[0] != window[1]
            && window[0] != window[2]
            && window[0] != window[3]
            && window[1] != window[2]
            && window[1] != window[3]
            && window[2] != window[3]
        {
            break;
        }
        pos += 1;
    }

    pos + size as u32
}

fn process_input_second(input: &str) -> u32 {
    let size = 14;
    let mut pos = 0;
    let chars = input.chars().into_iter().collect::<Vec<char>>();
    let windows = chars.windows(size);
    for window in windows {
        if window.iter().collect::<HashSet<&char>>().len() == size {
            break;
        }
        pos += 1;
    }

    pos + size as u32
}
