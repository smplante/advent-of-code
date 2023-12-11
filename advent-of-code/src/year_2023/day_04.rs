mod v1;

aoc_pm::run!(2023, 04, 13, 30);

pub fn part_1(input: &str) -> Option<u32> {
    v1::part_1(input)
}
pub fn part_2(input: &str) -> Option<u32> {
    v1::part_2(input)
}

#[cfg(test)]
mod tests {
    use crate::year_2023::{day_04, Data};

    #[test]
    fn part_1_sample() {
        let d = Data::get("day_04_sample_2")
            .expect("src/year_2023/day_04_sample_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_04::part_1(input), Some(13));
    }

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_04_part_1")
            .expect("src/year_2023/day_04_sample_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_04::part_1(input), Some(22674));
    }

    #[test]
    fn part_2_sample() {
        let d = Data::get("day_04_sample_1")
            .expect("src/year_2023/day_04_sample_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_04::part_2(input), Some(30));
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_04_part_2")
            .expect("src/year_2023/day_04_sample_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_04::part_2(input), Some(5747443));
    }
}
