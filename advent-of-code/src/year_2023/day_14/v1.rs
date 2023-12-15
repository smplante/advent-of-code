use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

pub fn part_1(input: &str) -> Option<usize> {
    let mut dish = input.lines().map(|l| l.as_bytes().to_vec()).collect_vec();

    tilt_north(&mut dish);

    Some(calculate_load(&dish))
}

pub fn part_2(input: &str) -> Option<usize> {
    let mut dish = input.lines().map(|l| l.as_bytes().to_vec()).collect_vec();

    let mut seen = BTreeMap::new();
    let mut loops = BTreeSet::new();
    let mut last_loop = 0;

    let loop_size = 1_000_000_000;

    for i in 1..=loop_size {
        cycle(&mut dish);
        if seen.contains_key(&dish) {
            if loops.contains(seen.get(&dish).unwrap()) {
                last_loop = i;
                break;
            }
            loops.insert(*seen.get(&dish).unwrap());
        }

        if !seen.contains_key(&dish) {
            seen.insert(dish.clone(), i);
            loops.clear();
        }
    }

    for _ in 0..(loop_size - last_loop) % loops.len() {
        cycle(&mut dish);
    }

    Some(calculate_load(&dish))
}

fn cycle(dish: &mut [Vec<u8>]) {
    tilt_north(dish);
    tilt_lat(dish, true);
    tilt_south(dish);
    tilt_lat(dish, false);
}

fn tilt_north(dish: &mut [Vec<u8>]) {
    for col in 0..dish[0].len() {
        let mut post_rock_position = 0;
        let mut round_rocks = 0;

        for row in 0..dish.len() {
            match dish[row][col] {
                b'#' => {
                    for i in (post_rock_position..).take(round_rocks) {
                        dish[i][col] = b'O';
                    }
                    for i in (post_rock_position + round_rocks..row) {
                        dish[i][col] = b'.';
                    }
                    post_rock_position = row + 1;
                    round_rocks = 0;
                }
                b'O' => {
                    round_rocks += 1;
                }
                _ => {}
            }
        }
        for i in (post_rock_position..).take(round_rocks) {
            dish[i][col] = b'O';
        }
        for i in (post_rock_position + round_rocks..dish.len()) {
            dish[i][col] = b'.';
        }
    }
}

fn tilt_south(dish: &mut [Vec<u8>]) {
    for col in 0..dish[0].len() {
        let mut post_rock_position = dish.len();
        let mut round_rocks = 0;

        for row in (0..dish.len()).rev() {
            match dish[row][col] {
                b'#' => {
                    for i in
                        (row + 1..post_rock_position).take(post_rock_position - row - round_rocks)
                    {
                        dish[i][col] = b'.';
                    }
                    for i in (post_rock_position - round_rocks)..post_rock_position {
                        dish[i][col] = b'O';
                    }
                    post_rock_position = row;
                    round_rocks = 0;
                }
                b'O' => {
                    round_rocks += 1;
                }
                _ => {}
            }
        }
        for i in (0..post_rock_position).take(post_rock_position - round_rocks) {
            dish[i][col] = b'.';
        }
        for i in (post_rock_position - round_rocks)..post_rock_position {
            dish[i][col] = b'O';
        }
    }
}

fn tilt_lat(dish: &mut [Vec<u8>], west: bool) {
    dish.iter_mut().for_each(|row| {
        let rocks = row.iter().positions(|b| b == &b'#').collect_vec();
        let mut post_rock = 0;
        for i in 0..rocks.len() {
            row[post_rock..rocks[i]].sort();
            if west {
                row[post_rock..rocks[i]].reverse();
            }
            post_rock = rocks[i] + 1;
        }
        let end = row.len();
        row[post_rock..end].sort();
        if west {
            row[post_rock..end].reverse();
        }
    });
}

fn calculate_load(dish: &[Vec<u8>]) -> usize {
    let mut load = vec![0; dish.len()];

    for col in 0..dish[0].len() {
        for row in 0..dish.len() {
            if dish[row][col] == b'O' {
                load[row] += 1;
            }
        }
    }

    load.iter()
        .rev()
        .enumerate()
        .map(|(f, l)| l * (f + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::year_2023::{day_14::v1::*, Data};

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_14_part_1")
            .expect("src/year_2023/day_14_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_1(input), Some(110_565));
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_14_part_2")
            .expect("src/year_2023/day_14_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_2(input), Some(89_845));
    }
}
