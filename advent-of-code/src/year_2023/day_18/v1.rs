use itertools::Itertools;

pub fn part_1(input: &str) -> Option<i64> {
    let instructions = input.lines().filter_map(|l| {
        let mut parts = l.split_whitespace();
        let dir = match parts.next()? {
            "R" => 0,
            "D" => 1,
            "L" => 2,
            "U" => 3,
            _ => unreachable!(),
        };
        let steps = parts.next()?.parse::<i64>().ok()?;

        Some((dir, steps))
    });

    let mut vertices = vec![(0, 0)];
    let mut pos = (0, 0);
    let mut boundary = 0;

    for (dir, steps) in instructions {
        boundary += steps;
        match dir {
            0 => {
                pos = (pos.0 + steps, pos.1);
                vertices.push(pos);
            }
            1 => {
                pos = (pos.0, pos.1 + steps);
                vertices.push(pos);
            }
            2 => {
                pos = (pos.0 - steps, pos.1);
                vertices.push(pos);
            }
            3 => {
                pos = (pos.0, pos.1 - steps);
                vertices.push(pos);
            }
            _ => unreachable!(),
        }
    }
    vertices.push((0, 0));

    let area = vertices
        .iter()
        .tuple_windows()
        .map(|(l, r)| l.0 * r.1 - l.1 * r.0)
        .sum::<i64>()
        / 2;

    let internal = area + boundary / 2 + 1;

    Some(internal)
}

pub fn part_2(input: &str) -> Option<i64> {
    let instructions = input.lines().filter_map(|l| {
        let mut parts = l.split_whitespace();
        let dir_steps = parts
            .last()?
            .trim_matches(|c| c == '(' || c == ')' || c == '#');

        let mut steps: i64 = 0;
        let mut dir = 0;
        for (i, c) in dir_steps.chars().enumerate() {
            if i == 5 {
                dir = match c {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '3' => 3,
                    _ => unreachable!(),
                };
                break;
            }
            steps += 16i64.pow(4 - i as u32)
                * match c {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    'a' => 10,
                    'b' => 11,
                    'c' => 12,
                    'd' => 13,
                    'e' => 14,
                    'f' => 15,
                    _ => unreachable!(),
                };
        }

        Some((dir, steps))
    });

    let mut vertices = vec![(0, 0)];
    let mut pos = (0, 0);
    let mut boundary = 0;

    for (dir, steps) in instructions {
        boundary += steps;
        match dir {
            0 => {
                pos = (pos.0 + steps, pos.1);
                vertices.push(pos);
            }
            1 => {
                pos = (pos.0, pos.1 + steps);
                vertices.push(pos);
            }
            2 => {
                pos = (pos.0 - steps, pos.1);
                vertices.push(pos);
            }
            3 => {
                pos = (pos.0, pos.1 - steps);
                vertices.push(pos);
            }
            _ => unreachable!(),
        }
    }
    vertices.push((0, 0));

    let area = vertices
        .iter()
        .tuple_windows()
        .map(|(l, r)| l.0 * r.1 - l.1 * r.0)
        .sum::<i64>()
        / 2;

    let internal = area + boundary / 2 + 1;

    Some(internal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::year_2023::Data;

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_18_part_1")
            .expect("src/year_2023/day_18_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_1(input), Some(72_821));
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_18_part_2")
            .expect("src/year_2023/day_18_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_2(input), Some(127_844_509_405_501));
    }
}
