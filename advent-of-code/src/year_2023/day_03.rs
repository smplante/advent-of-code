mod v1;

aoc_pm::run!(2023, 03, 4361, 467835);

pub fn part_1(input: &str) -> Option<u32> {
    v1::part_1(input)
}
pub fn part_2(input: &str) -> Option<u32> {
    v1::part_2(input)
}

#[cfg(test)]
mod tests {
    use crate::year_2023::{day_03, Data};

    #[test]
    fn part_1_sample() {
        let d = Data::get("day_03_sample_2")
            .expect("src/year_2023/day_03_sample_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_03::part_1(input), Some(4361));
    }
    #[test]
    fn part_1_actual() {
        let d = Data::get("day_03_part_1")
            .expect("src/year_2023/day_03_sample_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_03::part_1(input), Some(544433));
    }

    #[test]
    fn part_2_sample() {
        let d = Data::get("day_03_sample_1")
            .expect("src/year_2023/day_03_sample_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_03::part_2(input), Some(467835));
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_03_part_2")
            .expect("src/year_2023/day_03_sample_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_03::part_2(input), Some(76314915));
    }
}
