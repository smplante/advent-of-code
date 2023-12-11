use itertools::Itertools;

/// # Panics
pub fn part_1(input: &str) -> u32 {
    input
        .split('\n')
        .filter_map(parse_line_to_card)
        .sorted_by(|(lh, _), (rh, _)| lh.cmp(rh))
        .enumerate()
        .map(|(i, (_, v))| v * (i as u32 + 1))
        .sum::<u32>()
}

type CardV2 = u32;
fn parse_line_to_card(line: &str) -> Option<(CardV2, u32)> {
    let (c, v) = line.split_once(' ')?;
    let mut seen = [0; 15];
    let card: CardV2 = c
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

    let card: CardV2 = match hand[..] {
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

pub fn part_2(input: &str) -> u32 {
    input
        .split('\n')
        .filter_map(parse_line_to_card_2)
        .sorted_by(|(lh, _), (rh, _)| lh.cmp(rh))
        .enumerate()
        .map(|(i, (_, v))| v * (i as u32 + 1))
        .sum::<u32>()
}
pub fn parse_line_to_card_2(line: &str) -> Option<(CardV2, u32)> {
    let (c, v) = line.split_once(' ')?;
    let mut seen = [0; 15];
    let card: CardV2 = c
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

    let card: CardV2 = match hand[..] {
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
    use crate::year_2023::{day_07::v2::*, Data};

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
