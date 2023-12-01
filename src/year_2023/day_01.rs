use crate::year_2023::Data;
use rayon::prelude::*;

pub fn run(){
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

const NUMS: [(&[u8], &str); 9] = [
    ("one".as_bytes(), "1"),
    ("two".as_bytes(), "2"),
    ("three".as_bytes(), "3"),
    ("four".as_bytes(), "4"),
    ("five".as_bytes(), "5"),
    ("six".as_bytes(), "6"),
    ("seven".as_bytes(), "7"),
    ("eight".as_bytes(), "8"),
    ("nine".as_bytes(), "9"),
];

pub fn part_1_rayon(input: &str) -> u32 {
    input
        .lines()
        .par_bridge()
        .filter(|l| !l.is_empty())
        .map(|l| parse_line_1(l))
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

pub fn part_2_rayon(input: &str) -> u32 {
    input
        .lines()
        .par_bridge()
        .filter(|l| !l.is_empty())
        .map(|l| parse_line_2(l))
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

fn parse_line_1(line: &str) -> u32 {
    let mut n: Vec<&str> = Vec::new();
    let mut idx = 0;
    let line_bytes = line.as_bytes();

    while idx < line_bytes.len() {
        if line_bytes[idx].is_ascii_digit() {
            n.push(std::str::from_utf8(&line_bytes[idx..(idx + 1)]).expect("a digit"));
        }

        idx += 1;
    }

    format!("{}{}", n.first().unwrap(), n.last().unwrap())
        .parse::<u32>()
        .expect("to be a number")
}

fn parse_line_2(line: &str) -> u32 {
    let mut n: Vec<&str> = Vec::new();
    let mut idx = 0;
    let line_bytes = line.as_bytes();

    'outer: while idx < line_bytes.len() {
        let rest = &line_bytes[idx..];

        for (word, value) in NUMS {
            if rest.starts_with(word) {
                n.push(value);
                idx += word.len() - 1;
                continue 'outer;
            }
        }

        if rest[0].is_ascii_digit() {
            n.push(std::str::from_utf8(&rest[..1]).expect("a digit"));
        }

        idx += 1;
    }

    format!("{}{}", n.first().unwrap(), n.last().unwrap())
        .parse::<u32>()
        .expect("to be a number")
}
