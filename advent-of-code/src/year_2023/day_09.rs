mod v1;

aoc_pm::run!(2023, 09, 114, 2);

pub fn part_1(input: &str) -> i32 {
    v1::part_1(input)
}

pub fn part_2(input: &str) -> i32 {
    v1::part_2(input)
}

#[cfg(test)]
mod tests {
    use crate::year_2023::{day_09::*, Data};

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_09_part_1")
            .expect("src/year_2023/day_09_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_1(input), 1_696_140_818);
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_09_part_2")
            .expect("src/year_2023/day_09_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_2(input), 1_152);
    }
}
