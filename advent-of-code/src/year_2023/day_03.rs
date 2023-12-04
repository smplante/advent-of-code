use rayon::prelude::*;
use std::io::BufRead;
aoc_pm::make_run!(2023_03);

pub fn part_1(input: &str) -> u32 {
    let lb = input.as_bytes();
    let mut width = None;

    let mut serials = vec![vec![]];
    let mut parts = vec![vec![]];

    let mut num = Vec::new();

    let mut row = 0;
    let mut start_end: Option<(usize, usize)> = None;

    let mut sum = 0;

    for (i, &b) in lb.iter().enumerate() {
        let i = width.map_or(i, |w| i % w);

        match b {
            b if b >= b'0' && b <= b'9' => {
                num.push(b - b'0');
                start_end = Some(start_end.map_or((i, i), |se| (se.0, i)));
            }
            b'.' if (&num).len() > 0 => {
                let mut s = 0u32;
                for &n in num.iter() {
                    s = s * 10 + n as u32;
                }

                serials.get_mut(row).unwrap().push((s, start_end.unwrap()));
                start_end = None;
                num = Vec::new();
            }
            b'.' => {
                continue;
            }
            b'\n' => {
                width = width.or(Some(i + 1));
                if num.len() > 0 {
                    let mut s = 0u32;
                    for &n in num.iter() {
                        s = s * 10 + n as u32;
                    }

                    serials.get_mut(row).unwrap().push((s, start_end.unwrap()));
                    start_end = None;
                    num = Vec::new();
                }

                row += 1;
                serials.push(Vec::new());
                parts.push(Vec::new());
            }
            _ => {
                if num.len() > 0 {
                    let mut s = 0u32;
                    for &n in num.iter() {
                        s = s * 10 + n as u32;
                    }

                    serials.get_mut(row).unwrap().push((s, start_end.unwrap()));
                    start_end = None;
                    num = Vec::new();
                }

                parts.get_mut(row).unwrap().push((row, i))
            }
        }
    }

    for v in parts.iter() {
        for &(r, c) in v.iter() {
            let row_to_check = serials.get_mut(r - 1).unwrap();
            let mut to_remove = Vec::new();
            for pri in 0..row_to_check.len() {
                if in_range_of_part(c, row_to_check[pri].1, width.unwrap()) {
                    sum += row_to_check[pri].0;
                    to_remove.push(pri);
                }
            }
            for &i in to_remove.iter().rev() {
                row_to_check.remove(i);
            }

            let row_to_check = serials.get_mut(r).unwrap();
            let mut to_remove = Vec::new();
            for pri in 0..row_to_check.len() {
                if in_range_of_part(c, row_to_check[pri].1, width.unwrap()) {
                    sum += row_to_check[pri].0;
                    to_remove.push(pri);
                }
            }
            for &i in to_remove.iter().rev() {
                row_to_check.remove(i);
            }

            let row_to_check = serials.get_mut(r + 1).unwrap();
            let mut to_remove = Vec::new();
            for pri in 0..row_to_check.len() {
                if in_range_of_part(c, row_to_check[pri].1, width.unwrap()) {
                    sum += row_to_check[pri].0;
                    to_remove.push(pri);
                }
            }
            for &i in to_remove.iter().rev() {
                row_to_check.remove(i);
            }
        }
    }

    sum
}

pub fn part_2(input: &str) -> u32 {
    let lb = input.as_bytes();
    let mut width = None;

    let mut serials = vec![vec![]];
    let mut parts = vec![vec![]];

    let mut num = Vec::new();

    let mut row = 0;
    let mut start_end: Option<(usize, usize)> = None;

    let mut sum = 0;

    for (i, &b) in lb.iter().enumerate() {
        let i = width.map_or(i, |w| i % w);

        match b {
            b if b >= b'0' && b <= b'9' => {
                num.push(b - b'0');
                start_end = Some(start_end.map_or((i, i), |se| (se.0, i)));
            }
            b'.' if (&num).len() > 0 => {
                let mut s = 0u32;
                for &n in num.iter() {
                    s = s * 10 + n as u32;
                }

                serials.get_mut(row).unwrap().push((s, start_end.unwrap()));
                start_end = None;
                num = Vec::new();
            }
            b'.' => {
                continue;
            }
            b'\n' => {
                width = width.or(Some(i + 1));
                if num.len() > 0 {
                    let mut s = 0u32;
                    for &n in num.iter() {
                        s = s * 10 + n as u32;
                    }

                    serials.get_mut(row).unwrap().push((s, start_end.unwrap()));
                    start_end = None;
                    num = Vec::new();
                }

                row += 1;
                serials.push(Vec::new());
                parts.push(Vec::new());
            }
            b'*' => {
                if num.len() > 0 {
                    let mut s = 0u32;
                    for &n in num.iter() {
                        s = s * 10 + n as u32;
                    }

                    serials.get_mut(row).unwrap().push((s, start_end.unwrap()));
                    start_end = None;
                    num = Vec::new();
                }

                parts.get_mut(row).unwrap().push((row, i))
            }
            _ => {}
        }
    }

    for v in parts.iter() {
        for &(r, c) in v.iter() {
            let mut seen = Vec::new();

            let row_to_check = serials.get_mut(r - 1).unwrap();
            let mut to_remove = Vec::new();
            for pri in 0..row_to_check.len() {
                if in_range_of_part(c, row_to_check[pri].1, width.unwrap()) {
                    seen.push(row_to_check[pri].0);
                    to_remove.push(pri);
                }
            }

            let row_to_check = serials.get_mut(r).unwrap();
            let mut to_remove = Vec::new();
            for pri in 0..row_to_check.len() {
                if in_range_of_part(c, row_to_check[pri].1, width.unwrap()) {
                    seen.push(row_to_check[pri].0);
                    to_remove.push(pri);
                }
            }

            let row_to_check = serials.get_mut(r + 1).unwrap();
            let mut to_remove = Vec::new();
            for pri in 0..row_to_check.len() {
                if in_range_of_part(c, row_to_check[pri].1, width.unwrap()) {
                    seen.push(row_to_check[pri].0);
                    to_remove.push(pri);
                }
            }

            if seen.len() == 2 {
                sum += seen.iter().product::<u32>()
            }
        }
    }

    sum
}

fn in_range_of_part(part_col: usize, s_col_span: (usize, usize), width: usize) -> bool {
    if part_col <= s_col_span.1 && part_col >= s_col_span.0 {
        return true;
    } else if part_col == s_col_span.1 || part_col == s_col_span.0 {
        return true;
    } else if part_col > 0 && (part_col - 1 == s_col_span.0 || part_col - 1 == s_col_span.1) {
        return true;
    } else if part_col < width - 1 && (part_col + 1 == s_col_span.0 || part_col + 1 == s_col_span.1)
    {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::year_2023::{day_03, Data};

    #[test]
    fn part_1_sample() {
        let d = Data::get("day_03_sample_2")
            .expect("src/year_2023/day_03_sample_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_03::part_1(input), 4361);
    }

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_03_part_1")
            .expect("src/year_2023/day_03_sample_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_03::part_1(input), 544433);
    }

    #[test]
    fn part_2_sample() {
        let d = Data::get("day_03_sample_1")
            .expect("src/year_2023/day_03_sample_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_03::part_2(input), 467835);
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_03_part_2")
            .expect("src/year_2023/day_03_sample_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_03::part_2(input), 76314915);
    }
}
