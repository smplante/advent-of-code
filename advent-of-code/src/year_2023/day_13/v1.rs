use itertools::Itertools;

pub fn part_1(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .map(|block| block.lines().map(|l| l.as_bytes()).collect_vec())
            .map(|block| find_reflection(block, true))
            .sum(),
    )
}

pub fn part_2(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .map(|block| block.lines().map(|l| l.as_bytes()).collect_vec())
            .map(|block| find_reflection(block, false))
            .sum(),
    )
}

fn find_reflection(block: Vec<&[u8]>, part_1: bool) -> usize {
    let reflections = block.windows(2).positions(mostly_similar_row).collect_vec();
    'reflections: for reflection in reflections {
        let mut corrected = part_1;
        for p in 0..=reflection {
            if reflection + p + 1 >= block.len() {
                break;
            }
            for c in 0..block[0].len() {
                if corrected && block[reflection - p][c] != block[reflection + p + 1][c] {
                    continue 'reflections;
                }
                if block[reflection - p][c] != block[reflection + p + 1][c] {
                    corrected = true;
                }
            }
        }
        if corrected {
            return 100 * (reflection + 1);
        }
    }

    let reflections = mostly_similar_cols(&block);
    'reflections: for reflection in reflections {
        let mut corrected = part_1;
        for c in 0..=reflection {
            if reflection + c + 1 >= block[0].len() {
                break;
            }
            for r in 0..block.len() {
                if corrected && block[r][reflection - c] != block[r][reflection + c + 1] {
                    continue 'reflections;
                }
                if block[r][reflection - c] != block[r][reflection + c + 1] {
                    corrected = true;
                }
            }
        }
        if corrected {
            return reflection + 1;
        }
    }

    0
}

fn mostly_similar_row(rows: &[&[u8]]) -> bool {
    let mut corrected = false;
    for c in 0..rows[0].len() {
        if corrected && rows[0][c] != rows[1][c] {
            return false;
        }
        if rows[0][c] != rows[1][c] {
            corrected = true;
        }
    }

    true
}

fn mostly_similar_cols(block: &Vec<&[u8]>) -> Vec<usize> {
    let mut reflections = Vec::new();
    'row: for r in 0..block[0].len() - 1 {
        let mut corrected = false;
        for p in 0..block.len() {
            if corrected && block[p][r] != block[p][r + 1] {
                continue 'row;
            }
            if block[p][r] != block[p][r + 1] {
                corrected = true;
            }
        }
        reflections.push(r);
    }

    reflections
}

#[cfg(test)]
mod tests {
    use crate::year_2023::{day_13::v1::*, Data};

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_13_part_1")
            .expect("src/year_2023/day_13_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_1(input), Some(34_772));
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_13_part_2")
            .expect("src/year_2023/day_13_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_2(input), Some(35_554));
    }
}
