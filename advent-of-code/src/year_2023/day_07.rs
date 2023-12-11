mod v1;
mod v2;

aoc_pm::run!(2023, 07, 6440, 5905); // 6440 fails due to part 2 changes

pub fn part_1(input: &str) -> u32 {
    v2::part_1(input)
}
pub fn part_2(input: &str) -> u32 {
    v2::part_2(input)
}

#[cfg(test)]
mod tests {
    use crate::year_2023::{day_07::*, Data};

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_07_part_1")
            .expect("src/year_2023/day_07_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_1(input), 248_836_197);
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_07_part_2")
            .expect("src/year_2023/day_07_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_2(input), 251_195_607);
    }
}
