mod v1;

aoc_pm::run!(2023, 12, 21, 525152);

pub fn part_1(input: &str) -> Option<usize> {
    v1::part_1(input)
}
pub fn part_2(input: &str) -> Option<usize> {
    v1::part_2(input)
}

#[cfg(test)]
mod tests {
    use crate::year_2023::{day_12::*, Data};

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_12_part_1")
            .expect("src/year_2023/day_12part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert!(part_1(input).unwrap() > 5235); // 5235 is too low
        assert!(part_1(input).unwrap() > 5685); // 5685 is too low
        assert_eq!(part_1(input), Some(6949));
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_12_part_2")
            .expect("src/year_2023/day_12_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_2(input), Some(51_456_609_952_403));
    }
}
