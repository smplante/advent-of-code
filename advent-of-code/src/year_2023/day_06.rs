use itertools::Itertools;
aoc_pm::run!(2023, 06, 288, 71503);

/// # Panics
/// This function will panic if the input doesn't match the Advent of Code 2023 Day 05 puzzle input format
pub fn part_1(input: &str) -> u32 {
    let (t, d) = input.split_once('\n').unwrap();

    let times = parse_nums_1(t);
    let distances = parse_nums_1(d);

    times
        .into_iter()
        .zip(distances)
        .map(|(t, d)| {
            let mut i = d / t;
            while i * (t - i) <= d {
                i += 1;
            }

            1 + t - (2 * i)
        })
        .product()
}

fn parse_nums_1(nums: &str) -> Vec<u32> {
    nums.split_once(':')
        .map(|(_, data)| {
            data.split_whitespace()
                .filter_map(|d| d.parse::<u32>().ok())
                .collect_vec()
        })
        .unwrap()
}

/// # Panics
/// This function will panic if the input doesn't match the Advent of Code 2023 Day 05 puzzle input format
pub fn part_2(input: &str) -> u64 {
    let (t, d) = input.split_once('\n').unwrap();

    let t = parse_nums_2(t);
    let d = parse_nums_2(d);

    let mut i = solve_quadratic(1.0, t as f64, d as f64) as u64;
    while i * (t - i) > d {
        i += 1;
    }
    while i * (t - i) <= d {
        i += 1;
    }

    1 + t - (2 * i)
}

// d = x * (t - x)
// 0 = x^2 - tx + d
// x = (t +- sqrt(t^2 -4d)) / 2
/// This is fine to be lossy, since we'll just check based off the floored value anyway
/// (or walk to a new value if the conversion is too lossy)
fn solve_quadratic(a: f64, b: f64, c: f64) -> f64 {
    (b - f64::sqrt(b.powi(2) - (4.0 * a * c))) / (2.0 * a)
}

fn parse_nums_2(nums: &str) -> u64 {
    nums.split_once(':')
        .map(|(_, data)| data.split_whitespace().join("").parse::<u64>().unwrap())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::year_2023::{day_06, Data};

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_06_part_1")
            .expect("src/year_2023/day_06_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_06::part_1(input), 4_568_778);
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_06_part_2")
            .expect("src/year_2023/day_06_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_06::part_2(input), 28_973_936);
    }
}
