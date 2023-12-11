mod v1;

aoc_pm::run!(2023, 06, 288, 71503);

pub fn part_1(input: &str) -> u32 {
    v1::part_1(input)
}
pub fn part_2(input: &str) -> u64 {
    v1::part_2(input)
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
