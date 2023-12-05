use itertools::Itertools;

aoc_pm::run!(2023, 04, 13, 30);

pub fn part_1(input: &str) -> u32 {
    input.lines().map(parse_line_1).sum()
}

fn parse_line_1(line: &str) -> u32 {
    let (winning_numbers_raw, ticket_numbers) =
        line.split_once(':').unwrap().1.split_once('|').unwrap();
    let mut winning_numbers = [false; 100];
    for wn in winning_numbers_raw.split_ascii_whitespace() {
        winning_numbers[wn.parse::<usize>().unwrap()] = true;
    }

    2u32.pow(
        ticket_numbers
            .split_ascii_whitespace()
            .filter(|tn| winning_numbers[tn.parse::<usize>().unwrap()])
            .count() as u32,
    ) / 2
}

pub fn part_2(input: &str) -> u32 {
    let lines = input.split('\n').collect_vec();
    let mut games = vec![0; lines.len() + 1];

    for line in lines {
        let mut line_split = line.split([':', '|']);
        let game = line_split
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut winning_numbers = [false; 100];
        for wn in line_split.next().unwrap().split_ascii_whitespace() {
            winning_numbers[wn.parse::<usize>().unwrap()] = true;
        }

        games[game] += 1;

        for i in 1..=line_split
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .filter(|tn| winning_numbers[tn.parse::<usize>().unwrap()])
            .count()
        {
            games[game + i] += games[game];
        }
    }

    games.iter().sum()
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
        assert_eq!(day_04::part_1(input), 13);
    }

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_04_part_1")
            .expect("src/year_2023/day_04_sample_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_04::part_1(input), 22674);
    }

    #[test]
    fn part_2_sample() {
        let d = Data::get("day_04_sample_1")
            .expect("src/year_2023/day_04_sample_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_04::part_2(input), 30);
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_04_part_2")
            .expect("src/year_2023/day_04_sample_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_04::part_2(input), 5747443);
    }
}
