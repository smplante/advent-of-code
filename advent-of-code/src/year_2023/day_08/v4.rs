use crate::year_2023::day_08::v2;
use itertools::Itertools;
use rayon::prelude::*;

const A: u16 = 0u16;
const Z: u16 = 25u16;

#[allow(dead_code)]
pub fn part_1(input: &str) -> Option<u32> {
    v2::part_1(input)
}

#[allow(dead_code)]
pub fn part_2(input: &str) -> Option<u64> {
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

    Some(
        pos.par_iter()
            .map(|&s| {
                let mut pos = s;
                let mut steps = 0u64;
                loop {
                    for _ in 0..directions.len() {
                        pos =
                            positions[pos as usize][directions[steps as usize % directions.len()]];
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
            ),
    )
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
    use crate::year_2023::{day_08::v4::*, Data};

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
