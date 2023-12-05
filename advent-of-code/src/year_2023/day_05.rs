use itertools::Itertools;
aoc_pm::run!(2023, 05, 35, 0);

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

pub fn part_2(_input: &str) -> u64 {
    u64::MAX
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
        assert_eq!(day_05::part_1(input), 825516882);
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_05_part_2")
            .expect("src/year_2023/day_05_sample_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_05::part_2(input), 0);
    }
}
