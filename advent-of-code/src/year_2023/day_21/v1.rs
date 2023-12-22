use itertools::Itertools;
use std::collections::VecDeque;

pub fn part_1(input: &str) -> Option<usize> {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let start = find_start(&grid)?;
    let mut seen = vec![vec![0usize; grid[0].len()]; grid.len()];

    let mut pq = VecDeque::from([(start, 0)]);

    while !pq.is_empty() {
        let ((x, y), step) = pq.pop_front().unwrap();

        if step > 64 {
            break;
        }

        if grid[y][x] == '#' {
            continue;
        }

        if seen[y][x] == 0 || (seen[y][x] > 0 && step % 2 == 0 && seen[y][x] % 2 == 1) {
            seen[y][x] = step;
        } else {
            continue;
        }

        if x > 0 {
            pq.push_back(((x - 1, y), step + 1));
        }

        if y > 0 {
            pq.push_back(((x, y - 1), step + 1));
        }

        if x < grid[y].len() - 1 {
            pq.push_back(((x + 1, y), step + 1));
        }

        if y < grid.len() - 1 {
            pq.push_back(((x, y + 1), step + 1));
        }
    }

    seen.iter()
        .map(|r| r.iter().filter(|c| **c > 0 && **c % 2 == 0).count())
        .sum1()
}

pub fn part_2(input: &str) -> Option<usize> {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let start = find_start(&grid)?;
    let mut seen = vec![vec![0usize; grid[0].len()]; grid.len()];

    let mut pq = VecDeque::from([(start, 0)]);

    while !pq.is_empty() {
        let ((x, y), step) = pq.pop_front().unwrap();

        if step > grid.len() {
            break;
        }

        if grid[y][x] == '#' {
            continue;
        }

        if seen[y][x] == 0 {
            seen[y][x] = step;
        } else {
            continue;
        }

        if x == 0 || x == grid[y].len() - 1 || y == 0 || y == grid.len() - 1 {
            pq.push_back(((x, y), step + 1));
        }

        if x > 0 {
            pq.push_back(((x - 1, y), step + 1));
        }

        if y > 0 {
            pq.push_back(((x, y - 1), step + 1));
        }

        if x < grid[y].len() - 1 {
            pq.push_back(((x + 1, y), step + 1));
        }

        if y < grid.len() - 1 {
            pq.push_back(((x, y + 1), step + 1));
        }
    }

    let mut sum = 0;

    for (y, row) in seen.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if grid[y][x] == '#' {
                eprint!(" ## ");
            } else {
                if *cell > 0 {
                    eprint!(" {cell:0>2} ");
                    sum += 100 / cell
                } else {
                    eprint!(" .. ");
                }
            }
        }
        eprint!("\n");
    }

    seen.iter()
        .map(|r| r.iter().filter(|c| **c > 0 && **c % 2 == 0).count())
        .sum1()
}

fn find_start(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                return Some((x, y));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::year_2023::Data;

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_21_part_1")
            .expect("src/year_2023/day_21_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_1(input), Some(3617));
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_21_part_2")
            .expect("src/year_2023/day_21_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_2(input), Some(0));
    }
}
