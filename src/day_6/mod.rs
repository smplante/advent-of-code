use std::collections::HashSet;

mod inputs;

pub fn run() {
    println!("---------------------");
    println!("Advent of Code Day 06");
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
        "actual: first half: {:?}, second half: {:?}\n",
        result_actual_first, result_actual_second
    );
}

fn process_input_first(raw: &str) -> u32 {
    let size = 4;
    let mut pos = 0;
    let chars = raw.chars().into_iter().collect::<Vec<char>>();
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
        pos = pos + 1;
    }

    pos + size as u32
}

fn process_input_second(raw: &str) -> u32 {
    let size = 14;
    let mut pos = 0;
    let chars = raw.chars().into_iter().collect::<Vec<char>>();
    let windows = chars.windows(size);
    for window in windows {
        if window.iter().collect::<HashSet<&char>>().len() == size {
            break;
        }
        pos = pos + 1;
    }

    pos + size as u32
}
