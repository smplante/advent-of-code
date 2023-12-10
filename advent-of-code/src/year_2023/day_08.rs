use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
aoc_pm::run!(2023, 08, 2, 6);

pub fn part_1(input: &str) -> u32 {
    part_1_v2(input)
}
/// # Panics
/// This function will panic if the input doesn't match the Advent of Code 2023 Day 05 puzzle input format
pub fn part_1_v1(input: &str) -> u32 {
    let mut input = input.split_terminator('\n').filter(|s| !s.is_empty());
    let directions = input
        .next()
        .map(str::as_bytes)
        .unwrap()
        .iter()
        .map(|b| match b {
            b'L' => 0usize,
            b'R' => 1usize,
            _ => unreachable!(),
        })
        .collect_vec();

    let mut positions: HashMap<String, [String; 2]> = HashMap::new();
    for line in input {
        let line = line.replace(['=', '(', ')', ','], " ");
        let [pos, left, right] = line
            .split_whitespace()
            .filter(|l| !l.is_empty())
            .collect_vec()[..3]
        else {
            unreachable!();
        };

        positions.insert(pos.to_string(), [left.to_string(), right.to_string()]);
    }

    let mut pos = "AAA";
    let mut steps = 0;
    loop {
        pos = &positions.get(pos).unwrap()[directions[steps % directions.len()]];
        steps += 1;
        if pos == "ZZZ" {
            return steps as u32;
        }
    }
}
/// # Panics
/// This function will panic if the input doesn't match the Advent of Code 2023 Day 05 puzzle input format
pub fn part_1_v2(input: &str) -> u32 {
    let mut input = input.split_terminator('\n').filter(|s| !s.is_empty());
    let directions = input
        .next()
        .map(str::as_bytes)
        .unwrap()
        .iter()
        .map(|b| match b {
            b'L' => 0usize,
            b'R' => 1usize,
            _ => unreachable!(),
        })
        .collect_vec();

    let mut positions = vec![[0; 2]; pos_to_num("ZZZ") as usize + 1];
    for line in input {
        let p = &line.as_bytes()[0..3];
        let left = &line.as_bytes()[7..10];
        let right = &line.as_bytes()[12..15];

        let pb = posb_to_num(p);
        positions[pb as usize] = [posb_to_num(left), posb_to_num(right)];
    }

    let mut pos = pos_to_num("AAA");
    let mut steps = 0;
    loop {
        pos = positions[pos as usize][directions[steps % directions.len()]];
        steps += 1;
        if pos == pos_to_num("ZZZ") {
            return steps as u32;
        }
    }
}

pub fn part_2(input: &str) -> u64 {
    part_2_v5(input)
}

/// # Panics
/// This function will panic if the input doesn't match the Advent of Code 2023 Day 05 puzzle input format
pub fn part_2_v1(input: &str) -> usize {
    let mut input = input.split_terminator('\n').filter(|s| !s.is_empty());
    let directions = input
        .next()
        .map(str::as_bytes)
        .unwrap()
        .iter()
        .map(|b| match b {
            b'L' => 0usize,
            b'R' => 1usize,
            _ => unreachable!(),
        })
        .collect_vec();

    let mut pos = Vec::new();

    let mut positions: HashMap<String, [String; 2]> = HashMap::new();
    for line in input {
        let line = line.replace(['=', '(', ')', ','], " ");
        let [p, left, right] = line
            .split_whitespace()
            .filter(|l| !l.is_empty())
            .collect_vec()[..3]
        else {
            unreachable!();
        };

        let p = p.to_string();
        if p.ends_with('A') {
            pos.push(p.to_string());
        }
        positions.insert(p, [left.to_string(), right.to_string()]);
    }

    let mut loop_len = Vec::new();
    for i in 0..pos.len() {
        loop_len.push(vec![pos[i].clone()]);
    }

    let mut ends = vec![0usize; loop_len.len()];

    let mut steps = 0;
    loop {
        for _ in 0..directions.len() {
            for i in 0..pos.len() {
                pos[i] = positions.get(&pos[i]).unwrap()[directions[steps % directions.len()]]
                    .to_string();
                loop_len[i].push(pos[i].clone());
            }
            steps += 1;
            pos.iter()
                .enumerate()
                .filter(|(i, p)| p.ends_with('Z'))
                .for_each(|(i, p)| {
                    if ends[i] == 0 {
                        ends[i] = steps;
                    }
                });
            if ends.iter().filter(|&&e| e > 0).count() == ends.len() {
                let mut divs = HashSet::new();
                for &e in &ends {
                    for i in 2..=e {
                        if e % i == 0 {
                            divs.insert(i);
                            divs.insert(e / i);
                        }
                        if i * i > e && e > 100 {
                            break;
                        }
                    }
                }
                return divs.iter().product();
            }
        }
    }
}

/// # Panics
/// This function will panic if the input doesn't match the Advent of Code 2023 Day 05 puzzle input format
pub fn part_2_v2(input: &str) -> usize {
    let mut input = input.split_terminator('\n').filter(|s| !s.is_empty());
    let directions = input
        .next()
        .map(str::as_bytes)
        .unwrap()
        .iter()
        .map(|b| match b {
            b'L' => 0usize,
            b'R' => 1usize,
            _ => unreachable!(),
        })
        .collect_vec();

    let mut pos = Vec::new();

    let mut positions: HashMap<String, [String; 2]> = HashMap::new();
    for line in input {
        let line = line.replace(['=', '(', ')', ','], " ");
        let [p, left, right] = line
            .split_whitespace()
            .filter(|l| !l.is_empty())
            .collect_vec()[..3]
        else {
            unreachable!();
        };

        let p = p.to_string();
        if p.ends_with('A') {
            pos.push(p.to_string());
        }
        positions.insert(p, [left.to_string(), right.to_string()]);
    }

    // let mut loop_len = Vec::new();
    // for i in 0..pos.len() {
    //     loop_len.push(vec![pos[i].clone()]);
    // }

    let mut ends = vec![0usize; pos.len()];

    let mut steps = 0;
    loop {
        for _ in 0..directions.len() {
            for i in 0..pos.len() {
                pos[i] = positions.get(&pos[i]).unwrap()[directions[steps % directions.len()]]
                    .to_string();
                // loop_len[i].push(pos[i].clone());
            }
            steps += 1;
            pos.iter()
                .enumerate()
                .filter(|(i, p)| p.ends_with('Z'))
                .for_each(|(i, p)| {
                    if ends[i] == 0 {
                        ends[i] = steps;
                    }
                });
            if ends.iter().filter(|&&e| e > 0).count() == ends.len() {
                let mut divs = HashSet::new();
                for &e in &ends {
                    for i in 2..=e {
                        if e % i == 0 {
                            divs.insert(i);
                            divs.insert(e / i);
                        }
                        if i * i > e && e > 100 {
                            break;
                        }
                    }
                }
                return divs.iter().product();
            }
        }
    }
}

const A: u16 = 0u16;
const Z: u16 = 25u16;

/// # Panics
/// This function will panic if the input doesn't match the Advent of Code 2023 Day 05 puzzle input format
pub fn part_2_v3(input: &str) -> usize {
    let input = input.replace(['=', '(', ')', ','], " ");
    let mut input = input.split_terminator('\n').filter(|s| !s.is_empty());
    let directions = input
        .next()
        .map(str::as_bytes)
        .unwrap()
        .iter()
        .map(|b| match b {
            b'L' => 0usize,
            b'R' => 1usize,
            _ => unreachable!(),
        })
        .collect_vec();

    let mut pos = Vec::new();

    let mut positions: HashMap<u16, [u16; 2]> = HashMap::new();
    for line in input {
        let [p, left, right] = line
            .split_whitespace()
            .filter(|l| !l.is_empty())
            .collect_vec()[..3]
        else {
            unreachable!();
        };

        let pb = pos_to_num(p);
        if pb << 11 == A << 11 {
            pos.push(pb);
        }
        positions.insert(pb, [pos_to_num(left), pos_to_num(right)]);
    }

    let mut ends = vec![0usize; pos.len()];

    let mut steps = 0;
    loop {
        for _ in 0..directions.len() {
            for i in 0..pos.len() {
                pos[i] = positions.get(&pos[i]).unwrap()[directions[steps % directions.len()]];
            }
            steps += 1;
            pos.iter()
                .enumerate()
                .filter(|(_, &p)| p << 11 == Z << 11)
                .for_each(|(i, _)| {
                    if ends[i] == 0 {
                        ends[i] = steps;
                    }
                });
            if ends.iter().filter(|&&e| e > 0).count() == ends.len() {
                let mut divs = HashSet::new();
                for &e in &ends {
                    for i in 2..=e {
                        if e % i == 0 {
                            divs.insert(i);
                            divs.insert(e / i);
                        }
                        if i * i > e && e > 100 {
                            break;
                        }
                    }
                }
                return divs.iter().product();
            }
        }
    }
}

/// # Panics
/// This function will panic if the input doesn't match the Advent of Code 2023 Day 05 puzzle input format
pub fn part_2_v4(input: &str) -> u64 {
    let mut input = input.split_terminator('\n').filter(|s| !s.is_empty());
    let directions = input
        .next()
        .map(str::as_bytes)
        .unwrap()
        .iter()
        .map(|b| match b {
            b'L' => 0usize,
            b'R' => 1usize,
            _ => unreachable!(),
        })
        .collect_vec();

    let mut pos = Vec::new();

    let mut positions = vec![[0; 2]; pos_to_num("ZZZ") as usize + 1];
    for line in input {
        let p = &line.as_bytes()[0..3];
        let left = &line.as_bytes()[7..10];
        let right = &line.as_bytes()[12..15];

        let pb = posb_to_num(p);
        if pb << 11 == A << 11 {
            pos.push(pb);
        }
        positions[pb as usize] = [posb_to_num(left), posb_to_num(right)];
    }

    pos.par_iter()
        .map(|&s| {
            let mut pos = s;
            let mut steps = 0u64;
            loop {
                for _ in 0..directions.len() {
                    pos = positions[pos as usize][directions[steps as usize % directions.len()]];
                    steps += 1;
                    if pos << 11 == Z << 11 {
                        return steps;
                    }
                }
            }
        })
        .flat_map(|s| {
            let mut divs = Vec::new();
            for i in 2..=s {
                if s % i == 0 {
                    divs.push(i);
                    divs.push(s / i);
                }
                if i * i > s && s > 100 {
                    break;
                }
            }
            divs
        })
        .reduce(
            || 1u64,
            |acc, d| {
                if acc % d == 0 {
                    return acc;
                }
                acc * d
            },
        )
}

/// # Panics
/// This function will panic if the input doesn't match the Advent of Code 2023 Day 05 puzzle input format
pub fn part_2_v5(input: &str) -> u64 {
    let mut input = input.split_terminator('\n').filter(|s| !s.is_empty());
    let directions = input
        .next()
        .map(str::as_bytes)
        .unwrap()
        .iter()
        .map(|b| match b {
            b'L' => 0usize,
            b'R' => 1usize,
            _ => unreachable!(),
        })
        .collect_vec();

    let mut pos = Vec::new();

    let mut positions = vec![[0; 2]; pos_to_num("ZZZ") as usize + 1];
    for line in input {
        let p = &line.as_bytes()[0..3];
        let left = &line.as_bytes()[7..10];
        let right = &line.as_bytes()[12..15];

        let pb = posb_to_num(p);
        if pb << 11 == A << 11 {
            pos.push(pb);
        }
        positions[pb as usize] = [posb_to_num(left), posb_to_num(right)];
    }

    pos.par_iter()
        .map(|&s| {
            let mut pos = s;
            let mut steps = 0u64;
            loop {
                for _ in 0..directions.len() {
                    pos = positions[pos as usize][directions[steps as usize % directions.len()]];
                    steps += 1;
                    if pos << 11 == Z << 11 {
                        return steps;
                    }
                }
            }
        })
        .reduce(
            || 1,
            |acc, s| {
                let a = (acc * s) / gcd(s, acc);
                a
            },
        )
}

// Euclidean algorithm
fn gcd(a: u64, b: u64) -> u64 {
    if b != 0 {
        return gcd(b, a % b);
    }
    a
}

fn pos_to_num(p: &str) -> u16 {
    p.as_bytes()
        .iter()
        .fold(0u16, |acc, &b| (acc << 5) + (b - b'A') as u16)
}
fn posb_to_num(p: &[u8]) -> u16 {
    p.iter()
        .fold(0u16, |acc, &b| (acc << 5) + (b - b'A') as u16)
}

#[cfg(test)]
mod tests {
    use crate::year_2023::{day_08::*, Data};

    #[test] // fails due to part 2 changes
    fn part_1_actual() {
        let d = Data::get("day_08_part_1")
            .expect("src/year_2023/day_08_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_1(input), 11_309);
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_08_part_2")
            .expect("src/year_2023/day_08_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_2(input), 13_740_108_158_591);
    }
}
