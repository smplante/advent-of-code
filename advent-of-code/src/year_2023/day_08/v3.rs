use crate::year_2023::day_08::v2;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
pub fn part_1(input: &str) -> Option<u32> {
    v2::part_1(input)
}

const A: u16 = 0u16;
const Z: u16 = 25u16;

pub fn part_2(input: &str) -> Option<u64> {
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
                return Some(divs.iter().product::<usize>() as u64);
            }
        }
    }
}
fn pos_to_num(p: &str) -> u16 {
    p.as_bytes()
        .iter()
        .fold(0u16, |acc, &b| (acc << 5) + (b - b'A') as u16)
}

#[cfg(test)]
mod tests {
    use crate::year_2023::{day_08::v3::*, Data};

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_08_part_1")
            .expect("src/year_2023/day_08_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_1(input), Some(11_309));
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_08_part_2")
            .expect("src/year_2023/day_08_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_2(input), Some(13_740_108_158_591));
    }
}
