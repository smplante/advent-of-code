use itertools::Itertools;
use std::collections::{HashMap, HashSet};
aoc_pm::run!(2023, 08, 2, 6);

/// # Panics
/// This function will panic if the input doesn't match the Advent of Code 2023 Day 05 puzzle input format
pub fn part_1(input: &str) -> u32 {
    let mut input = input.split_terminator('\n').filter(|s| !s.is_empty());
    let directions = input
        .next()
        .map(str::as_bytes)
        .unwrap()
        .into_iter()
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
pub fn part_2(input: &str) -> usize {
    let mut input = input.split_terminator('\n').filter(|s| !s.is_empty());
    let directions = input
        .next()
        .map(str::as_bytes)
        .unwrap()
        .into_iter()
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
                    eprintln!("start {i} reached end {p} in {steps} steps");
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
