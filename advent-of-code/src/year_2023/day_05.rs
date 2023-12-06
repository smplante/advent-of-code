use itertools::Itertools;
aoc_pm::run!(2023, 05, 35, 46);

/// # Panics
/// This function will panic if the input doesn't match the Advent of Code 2023 Day 05 puzzle input format
pub fn part_1(input: &str) -> u64 {
    let mut lines = input.split('\n');

    let mut srcs = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|chunk| chunk.parse::<u64>().ok())
        .collect_vec();

    let mut lines = lines.collect_vec();

    assert_eq!(lines.remove(0), "");
    assert!(lines.remove(0).contains("map:"));

    let mut dsts = vec![u64::MAX; srcs.len()];
    while !lines.is_empty() {
        let [dst, src, rng] = lines
            .remove(0)
            .split_ascii_whitespace()
            .filter_map(|num| num.parse::<u64>().ok())
            .collect_vec()[..]
        else {
            for (idx, &src) in srcs.iter().enumerate() {
                if dsts[idx] == u64::MAX {
                    dsts[idx] = src;
                }
            }

            if lines.is_empty() {
                break;
            }

            srcs = dsts;
            dsts = vec![u64::MAX; srcs.len()];
            assert!(lines.remove(0).contains("map:"));
            continue;
        };

        for (idx, source) in srcs.iter().enumerate() {
            if (src..(src + rng)).contains(source) {
                dsts[idx] = (source - src) + dst;
            }
        }
    }

    dsts.into_iter().min().unwrap()
}

/// # Panics
/// This function will panic if the input doesn't match the Advent of Code 2023 Day 05 puzzle input format
pub fn part_2(input: &str) -> u64 {
    let mut lines = input.split('\n');

    let mut old_ranges = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let start = chunk.next().unwrap().parse::<u64>().unwrap();
            let range = chunk.next().unwrap().parse::<u64>().unwrap();
            (start, start + range)
        })
        .collect_vec();

    let mut lines = lines.collect_vec();

    assert!(lines.remove(0).is_empty());
    assert!(lines.remove(0).contains("map:"));

    let mut new_ranges = Vec::new();
    while !lines.is_empty() {
        let [dst, src, rng] = lines
            .remove(0)
            .split_ascii_whitespace()
            .filter_map(|num| num.parse::<u64>().ok())
            .collect_vec()[..]
        else {
            for &source in &old_ranges {
                new_ranges.push(source);
            }

            if lines.is_empty() {
                break;
            }

            old_ranges = new_ranges;
            new_ranges = Vec::new();
            assert!(lines.remove(0).contains("map:"));
            continue;
        };

        let mut to_remove = Vec::new();
        let mut to_add = Vec::new();
        for (idx, &(old_start, old_end)) in old_ranges.iter().enumerate() {
            let src_range = src..=(src + rng);
            let og_range = old_start..old_end;
            if src_range.contains(&old_start) && src_range.contains(&old_end) {
                let new_start = dst + old_start - src;
                let new_end = new_start + (old_end - old_start);
                new_ranges.push((new_start, new_end));
                to_remove.push(idx);
            } else if src_range.contains(&old_start) && !src_range.contains(&old_end) {
                let diff = old_start - src;
                let new_start = dst + diff;
                let new_end = dst + rng;
                new_ranges.push((new_start, new_end));
                let leftover_start = old_start + rng - diff;
                let leftover_end = old_end;
                to_add.push((leftover_start, leftover_end));
                to_remove.push(idx);
            } else if !src_range.contains(&old_start) && src_range.contains(&old_end) {
                let diff = old_end - src;
                let new_start = dst;
                let new_end = new_start + diff;
                new_ranges.push((new_start, new_end));
                let leftover_start = old_start;
                let leftover_end = src;
                to_add.push((leftover_start, leftover_end));
                to_remove.push(idx);
            } else if og_range.contains(&src) && og_range.contains(&(src + rng)) {
                let diff = old_end - src;
                let new_start = dst + diff;
                let new_end = new_start + rng;
                new_ranges.push((new_start, new_end));
                let leftover_start = old_start;
                let leftover_end = src;
                to_add.push((leftover_start, leftover_end));
                let leftover_start = src + rng;
                let leftover_end = old_end;
                if leftover_start < leftover_end {
                    to_add.push((leftover_start, leftover_end));
                }
                to_remove.push(idx);
            }
        }
        for &idx in to_remove.iter().rev() {
            old_ranges.remove(idx);
        }
        for &range in to_add.iter().rev() {
            old_ranges.push(range);
        }
    }

    let mut smallest = u64::MAX;
    for (start, _) in &new_ranges {
        if start < &smallest {
            smallest = *start;
        }
    }

    smallest
}

#[cfg(test)]
mod tests {
    use crate::year_2023::{day_05, Data};

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_05_part_1")
            .expect("src/year_2023/day_05_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_05::part_1(input), 825_516_882);
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_05_part_2")
            .expect("src/year_2023/day_05_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_05::part_2(input), 136_096_660);
    }
}
