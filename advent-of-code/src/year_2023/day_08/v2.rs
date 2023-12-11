use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn part_1(input: &str) -> u32 {
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

    let target = pos_to_num("ZZZ");
    let mut pos = pos_to_num("AAA");
    let mut steps = 0;
    loop {
        pos = positions[pos as usize][directions[steps % directions.len()]];
        steps += 1;
        if pos == target {
            return steps as u32;
        }
    }
}

#[allow(dead_code)]
pub fn part_2(input: &str) -> u64 {
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

    let mut ends = vec![0usize; pos.len()];

    let mut steps = 0;
    loop {
        for _ in 0..directions.len() {
            for i in 0..pos.len() {
                pos[i] = positions.get(&pos[i]).unwrap()[directions[steps % directions.len()]]
                    .to_string();
            }
            steps += 1;
            pos.iter()
                .enumerate()
                .filter(|(_, p)| p.ends_with('Z'))
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
                return divs.iter().product::<usize>() as u64;
            }
        }
    }
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
    use crate::year_2023::{day_08::v2::*, Data};

    #[test]
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
