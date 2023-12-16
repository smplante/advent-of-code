use itertools::Itertools;
use std::collections::VecDeque;

const BUCKET_COUNT: usize = 256;

pub fn part_1(input: &str) -> Option<usize> {
    input
        .split(',')
        .map(|w| {
            w.as_bytes()
                .iter()
                .map(|b| *b as usize)
                .fold(0, |acc, b| ((acc + b) * 17) % BUCKET_COUNT)
        })
        .sum1()
}

pub fn part_2(input: &str) -> Option<usize> {
    let mut boxes = vec![VecDeque::new(); BUCKET_COUNT];

    input.split(',').for_each(|w| {
        let parts = w.split_terminator(['=', '-']).collect_vec();
        let hash = parts[0]
            .as_bytes()
            .iter()
            .map(|b| *b as usize)
            .fold(0, |acc, b| ((acc + b) * 17) % BUCKET_COUNT);
        let pos = boxes[hash].iter().find_position(|(l, _)| *l == parts[0]);
        if parts.len() > 1 {
            if let Some((i, _)) = pos {
                boxes[hash][i].1 = parts[1].parse::<usize>().unwrap();
            } else {
                boxes[hash].push_back((parts[0], parts[1].parse::<usize>().unwrap()));
            }
        } else if let Some((i, _)) = pos {
            boxes[hash].remove(i);
        }
    });

    Some(boxes.iter().enumerate().fold(0, |acc, (b, dq)| {
        acc + dq
            .iter()
            .enumerate()
            .fold(0, |acc, (s, (_, fl))| acc + (b + 1) * (s + 1) * fl)
    }))
}

#[cfg(test)]
mod tests {
    use crate::year_2023::{day_15::v1::*, Data};

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_15_part_1")
            .expect("src/year_2023/day_15_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_1(input), Some(520_500));
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_15_part_2")
            .expect("src/year_2023/day_15_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_2(input), Some(213_097));
    }
}
