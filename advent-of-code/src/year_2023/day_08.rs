mod v1;
mod v2;
mod v3;
mod v4;
mod v5;

aoc_pm::run!(2023, 08, 2, 6);

pub fn part_1(input: &str) -> u32 {
    v2::part_1(input)
}
pub fn part_2(input: &str) -> u64 {
    v3::part_2(input)
}
#[cfg(test)]
mod tests {
    use crate::year_2023::{day_08::*, Data};

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_08_part_1")
            .expect("src/year_2023/day_08_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_1(input), 11_309);
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_08_part_2")
            .expect("src/year_2023/day_08_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_2(input), 13_740_108_158_591);
    }
}
