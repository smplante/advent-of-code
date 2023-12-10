use crate::year_2023::day_07::Hands::{
    FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair,
};
use itertools::Itertools;
use std::cmp::Ordering;
// aoc_pm::run!(2023, 07, 6440, 5905); // 6440 fails due to part 2 changes
aoc_pm::run!(2023, 07, 6440, 5905); // 6440 fails due to part 2 changes

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
struct Card(u8);

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        match value {
            "A" => Card(14),
            "K" => Card(13),
            "Q" => Card(12),
            "J" => Card(0), // this breaks part 1
            "T" => Card(10),
            "9" => Card(9),
            "8" => Card(8),
            "7" => Card(7),
            "6" => Card(6),
            "5" => Card(5),
            "4" => Card(4),
            "3" => Card(3),
            "2" => Card(2),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Hands {
    FiveOfAKind([Card; 5]),
    FourOfAKind([Card; 5]),
    FullHouse([Card; 5]),
    ThreeOfAKind([Card; 5]),
    TwoPair([Card; 5]),
    OnePair([Card; 5]),
    HighCard([Card; 5]),
}

impl From<&str> for Hands {
    fn from(s: &str) -> Self {
        let mut h = [Card(0), Card(0), Card(0), Card(0), Card(0)];
        for (i, c) in s.chars().map(|c| Card::from(&*c.to_string())).enumerate() {
            h[i] = c;
        }

        let mut c = h.iter().fold([0u8; 15], |mut r, c| {
            r[c.0 as usize] += 1;
            r
        });

        if c[0] > 0 {
            let mut max_i = 14;
            let mut max = 0;
            for i in 1..c.len() {
                if c[i] > max {
                    max = c[i];
                    max_i = i;
                }
            }

            c[max_i] += c[0];
            c[0] = 0;
        }

        let v = c.iter().filter(|&&c| c > 0).sorted().rev().collect_vec();

        match v[..] {
            [5] => FiveOfAKind(h),
            [4, 1] => FourOfAKind(h),
            [3, 2] => FullHouse(h),
            [3, 1, 1] => ThreeOfAKind(h),
            [2, 2, 1] => TwoPair(h),
            [2, 1, 1, 1] => OnePair(h),
            _ => HighCard(h),
        }
    }
}

impl PartialEq<Self> for Hands {
    fn eq(&self, _: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd for Hands {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (FiveOfAKind(a), FiveOfAKind(b))
            | (FourOfAKind(a), FourOfAKind(b))
            | (FullHouse(a), FullHouse(b))
            | (ThreeOfAKind(a), ThreeOfAKind(b))
            | (TwoPair(a), TwoPair(b))
            | (OnePair(a), OnePair(b))
            | (HighCard(a), HighCard(b)) => a.partial_cmp(b),
            (FiveOfAKind(_), _) => Some(Ordering::Greater),
            (_, FiveOfAKind(_)) => Some(Ordering::Less),
            (FourOfAKind(_), _) => Some(Ordering::Greater),
            (_, FourOfAKind(_)) => Some(Ordering::Less),
            (FullHouse(_), _) => Some(Ordering::Greater),
            (_, FullHouse(_)) => Some(Ordering::Less),
            (ThreeOfAKind(_), _) => Some(Ordering::Greater),
            (_, ThreeOfAKind(_)) => Some(Ordering::Less),
            (TwoPair(_), _) => Some(Ordering::Greater),
            (_, TwoPair(_)) => Some(Ordering::Less),
            (OnePair(_), _) => Some(Ordering::Greater),
            (_, OnePair(_)) => Some(Ordering::Less),
        }
    }
}

/// # Panics
/// This function will panic if the input doesn't match the Advent of Code 2023 Day 05 puzzle input format
pub fn part_1(input: &str) -> u32 {
    part_1_v2(input)
}

/// # Panics
/// This function will panic if the input doesn't match the Advent of Code 2023 Day 05 puzzle input format
pub fn part_1_v1(input: &str) -> u32 {
    input
        .split('\n')
        .filter_map(|s| s.split_once(' '))
        .map(|(h, v)| (Hands::from(h), v.parse::<u32>().unwrap()))
        .sorted_by(|(lh, _), (rh, _)| lh.partial_cmp(rh).unwrap())
        .enumerate()
        .map(|(i, (_, v))| v * (i as u32 + 1))
        .sum::<u32>()
}

/// # Panics
/// This function will panic if the input doesn't match the Advent of Code 2023 Day 05 puzzle input format
pub fn part_1_v2(input: &str) -> u32 {
    input
        .split('\n')
        .filter_map(parse_line_to_card)
        .sorted_by(|(lh, _), (rh, _)| lh.cmp(rh))
        .enumerate()
        .map(|(i, (_, v))| v * (i as u32 + 1))
        .sum::<u32>()
}

type Card_v2 = u32;
fn parse_line_to_card(line: &str) -> Option<(Card_v2, u32)> {
    let (c, v) = line.split_once(' ')?;
    let mut seen = [0; 15];
    let card: Card_v2 = c
        .as_bytes()
        .iter()
        .map(|b| {
            match b {
                b'A' => 14,
                b'K' => 13,
                b'Q' => 12,
                b'J' => 11, // this breaks part 1
                b'T' => 10,
                b'9' => 9,
                b'8' => 8,
                b'7' => 7,
                b'6' => 6,
                b'5' => 5,
                b'4' => 4,
                b'3' => 3,
                b'2' => 2,
                _ => unreachable!(),
            }
        })
        .map(|c| {
            seen[c as usize] += 1;
            c
        })
        .reduce(|acc, c| (acc << 4) + c)?;

    let hand = seen.iter().filter(|&&c| c > 0).sorted().rev().collect_vec();

    let card: Card_v2 = match hand[..] {
        [5] => card + (6 << 20),
        [4, 1] => card + (5 << 20),
        [3, 2] => card + (4 << 20),
        [3, 1, 1] => card + (3 << 20),
        [2, 2, 1] => card + (2 << 20),
        [2, 1, 1, 1] => card + (1 << 20),
        _ => card + (0 << 20),
    };

    Some((card, v.parse::<u32>().ok()?))
}

/// # Panics
/// This function will panic if the input doesn't match the Advent of Code 2023 Day 05 puzzle input format
pub fn part_2(input: &str) -> u32 {
    part_2_v2(input)
}

/// # Panics
/// This function will panic if the input doesn't match the Advent of Code 2023 Day 05 puzzle input format
pub fn part_2_v1(input: &str) -> u32 {
    input
        .split('\n')
        .filter_map(|s| s.split_once(' '))
        .map(|(h, v)| (Hands::from(h), v.parse::<u32>().unwrap()))
        .sorted_by(|(lh, _), (rh, _)| lh.partial_cmp(rh).unwrap())
        .enumerate()
        .map(|(i, (_, v))| v * (i as u32 + 1))
        .sum::<u32>()
}

/// # Panics
/// This function will panic if the input doesn't match the Advent of Code 2023 Day 05 puzzle input format
pub fn part_2_v2(input: &str) -> u32 {
    input
        .split('\n')
        .filter_map(parse_line_to_card_2)
        .sorted_by(|(lh, _), (rh, _)| lh.cmp(rh))
        .enumerate()
        .map(|(i, (_, v))| v * (i as u32 + 1))
        .sum::<u32>()
}
pub fn parse_line_to_card_2(line: &str) -> Option<(Card_v2, u32)> {
    let (c, v) = line.split_once(' ')?;
    let mut seen = [0; 15];
    let card: Card_v2 = c
        .as_bytes()
        .iter()
        .map(|b| {
            match b {
                b'A' => 14,
                b'K' => 13,
                b'Q' => 12,
                b'J' => 0, // this breaks part 1
                b'T' => 10,
                b'9' => 9,
                b'8' => 8,
                b'7' => 7,
                b'6' => 6,
                b'5' => 5,
                b'4' => 4,
                b'3' => 3,
                b'2' => 2,
                _ => unreachable!(),
            }
        })
        .map(|c| {
            seen[c as usize] += 1;
            c
        })
        .reduce(|acc, c| (acc << 4) + c)?;

    if seen[0] > 0 {
        let mut max_i = 14;
        let mut max = 0;
        for i in 1..seen.len() {
            if seen[i] > max {
                max = seen[i];
                max_i = i;
            }
        }

        seen[max_i] += seen[0];
        seen[0] = 0;
    }

    let hand = seen.iter().filter(|&&c| c > 0).sorted().rev().collect_vec();

    let card: Card_v2 = match hand[..] {
        [5] => card + (6 << 20),
        [4, 1] => card + (5 << 20),
        [3, 2] => card + (4 << 20),
        [3, 1, 1] => card + (3 << 20),
        [2, 2, 1] => card + (2 << 20),
        [2, 1, 1, 1] => card + (1 << 20),
        _ => card,
    };

    Some((card, v.parse::<u32>().ok()?))
}

#[cfg(test)]
mod tests {
    use crate::year_2023::{day_07::*, Data};

    #[test]
    fn hands_test() {
        let (left, _) = parse_line_to_card_2("2AAAA 0").unwrap();
        let (right, _) = parse_line_to_card_2("3AAAA 0").unwrap();
        assert!(left < right);

        let (left, _) = parse_line_to_card_2("2A2AA 0").unwrap();
        let (right, _) = parse_line_to_card_2("3AAAA 0").unwrap();
        assert!(left < right);

        let (left, _) = parse_line_to_card_2("2A22A 0").unwrap();
        let (right, _) = parse_line_to_card_2("22AAA 0").unwrap();
        assert!(left > right);

        let (left, _) = parse_line_to_card_2("23456 0").unwrap();
        let (right, _) = parse_line_to_card_2("34567 0").unwrap();
        assert!(left < right);
    }

    #[test]
    fn hands_test_2() {
        let left = Hands::from("2AAAA");
        let right = Hands::from("3AAAA");
        assert!(left < right);

        let left = Hands::from("2A2AA");
        let right = Hands::from("3AAAA");
        assert!(left < right);

        let left = Hands::from("2A22A");
        let right = Hands::from("22AAA");
        assert!(left > right);

        let left = Hands::from("23456");
        let right = Hands::from("34567");
        assert!(left < right);
    }

    #[test] // fails due to part 2 changes
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
