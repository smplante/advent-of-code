mod v1;

aoc_pm::run!(2023, 11, 374, 82_000_210);

pub fn part_1(input: &str) -> Option<u32> {
    v1::part_1(input)
}
pub fn part_2(input: &str) -> Option<usize> {
    v1::part_2(input)
}

#[cfg(test)]
mod tests {
    use crate::year_2023::{day_11::*, Data};

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_11_part_1")
            .expect("src/year_2023/day_11_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_1(input), Some(9_599_070));
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_11_part_2")
            .expect("src/year_2023/day_11_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_2(input), Some(842_645_913_794));
    }
}
