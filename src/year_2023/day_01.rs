use crate::year_2023::Data;
use rayon::prelude::*;

pub fn run() {
    println!("--------------------------");
    println!("Advent of Code 2023 Day 01");
    println!("--------------------------\n");

    let sample_01_data_cow = Data::get("day_01_sample_1").expect("file to exist").data;
    let sample_01_data = std::str::from_utf8(&sample_01_data_cow).expect("to be a string");

    let sample_02_data_cow = Data::get("day_01_sample_2").expect("file to exist").data;
    let sample_02_data = std::str::from_utf8(&sample_02_data_cow).expect("to be a string");

    let part_01_data_cow = Data::get("day_01_part_1").expect("file to exist").data;
    let part_01_data = std::str::from_utf8(&part_01_data_cow).expect("to be a string");

    let part_02_data_cow = Data::get("day_01_part_1").expect("file to exist").data;
    let part_02_data = std::str::from_utf8(&part_02_data_cow).expect("to be a string");

    let start = std::time::SystemTime::now();
    println!(
        "sample:  first half: {}, second half: {}",
        part_1(sample_01_data),
        part_2(sample_02_data)
    );
    println!(
        "actual: first half: {}, second half: {}\n",
        part_1(part_01_data),
        part_2(part_02_data)
    );
    println!("Day 01 completed in: {:?}\n", start.elapsed().unwrap());
}

const NUMS: [(&[u8], u8); 9] = [
    (b"one", b'1'),
    (b"two", b'2'),
    (b"three", b'3'),
    (b"four", b'4'),
    (b"five", b'5'),
    (b"six", b'6'),
    (b"seven", b'7'),
    (b"eight", b'8'),
    (b"nine", b'9'),
];

pub fn part_1_rayon(input: &str) -> u32 {
    input
        .lines()
        .par_bridge()
        .filter(|l| !l.is_empty())
        .map(parse_line_1)
        .sum()
}

pub fn part_1(input: &str) -> u32 {
    let lines = input
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();
    let mut sum = 0;
    for line in lines {
        sum += parse_line_1(line);
    }

    sum
}

fn parse_line_1(line: &str) -> u32 {
    let mut n: Vec<u8> = Vec::new();
    let line_bytes = line.as_bytes();

    let mut idx = 0;
    while idx < line_bytes.len() {
        if line_bytes[idx].is_ascii_digit() {
            n.push(line_bytes[idx]);
            break;
        }

        idx += 1;
    }

    idx = line_bytes.len() - 1;
    loop {
        if line_bytes[idx].is_ascii_digit() {
            n.push(line_bytes[idx]);
            break;
        }

        idx -= 1;
    }

    // we need to convert ascii digit bytes to their actual numeric value
    // which is why we subtract the bytes of 0 from the first and last found digits
    (10 * (n[0] - b'0') + (n[1] - b'0')) as u32
}

pub fn part_2_rayon(input: &str) -> u32 {
    input
        .lines()
        .par_bridge()
        .filter(|l| !l.is_empty())
        .map(parse_line_2)
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    let lines = input
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();
    let mut sum = 0;
    for line in lines {
        sum += parse_line_2(line);
    }

    sum
}

fn parse_line_2(line: &str) -> u32 {
    let mut n: Vec<u8> = Vec::new();
    let line_bytes = line.as_bytes();

    let mut idx = 0;
    'outer: while idx < line_bytes.len() {
        let rest = &line_bytes[idx..];

        for (word, value) in NUMS {
            if rest.starts_with(word) {
                n.push(value);
                break 'outer;
            }
        }

        if rest[0].is_ascii_digit() {
            n.push(rest[0]);
            break 'outer;
        }

        idx += 1;
    }

    idx = line_bytes.len() - 1;
    'outer: loop {
        let rest = &line_bytes[..(idx + 1)];

        for (word, value) in NUMS {
            if rest.ends_with(word) {
                n.push(value);
                break 'outer;
            }
        }

        if rest[idx].is_ascii_digit() {
            n.push(rest[idx]);
            break 'outer;
        }

        idx -= 1;
    }

    // we need to convert ascii digit bytes to their actual numeric value
    // which is why we subtract the bytes of 0 from the first and last found digits
    (10 * (n[0] - b'0') + (n[1] - b'0')) as u32
}