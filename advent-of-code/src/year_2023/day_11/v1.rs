use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;

pub fn part_1(input: &str) -> Option<u32> {
    let mut universe = input
        .lines()
        .flat_map(|l| if l.contains('#') { vec![l] } else { vec![l, l] })
        .map(|l| l.as_bytes().to_vec())
        .collect_vec();

    let universe = expand_universe(&mut universe);

    let galaxy_combinations = universe
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter_map(|(x, g)| {
                    if *g == b'#' {
                        return Some((x, y));
                    }
                    None
                })
                .collect_vec()
        })
        .collect_vec();

    Some(
        galaxy_combinations
            .iter()
            .combinations(2)
            .map(dist_calc_vec)
            .sum::<usize>() as u32,
    )
}

fn dist_calc_vec(v: Vec<&(usize, usize)>) -> usize {
    let (a, b) = (v[0], v[1]);
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn expand_universe(universe: &mut Vec<Vec<u8>>) -> &Vec<Vec<u8>> {
    'row: for x in (0..universe[0].len()).rev() {
        for y in 0..universe.len() {
            if universe[y][x] != b'.' {
                continue 'row;
            }
        }

        for y in 0..universe.len() {
            universe[y].insert(x, b'.')
        }
    }

    universe
}

pub fn part_2(input: &str) -> Option<usize> {
    let gap_size = 1_000_000;
    let mut gap_y = HashSet::new();
    let mut gap_x = (0..input.find('\n')?).collect::<HashSet<usize>>();

    let mut galaxy_combinations = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            if !l.contains('#') {
                gap_y.insert(y);
            } else {
                l.match_indices("#").for_each(|(i, _)| {
                    gap_x.remove(&i);
                });
            }
            (y, l.as_bytes().to_vec())
        })
        .flat_map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter_map(|(x, g)| {
                    if *g == b'#' {
                        return Some((x, y));
                    }
                    None
                })
                .collect_vec()
        })
        .collect_vec();

    Some(
        galaxy_combinations
            .iter()
            .combinations(2)
            // .par_bridge()
            .map(|c| dist_calc_vec_gap(c, &gap_y, &gap_x, gap_size))
            .sum::<usize>(),
    )
}

fn dist_calc_vec_gap(
    v: Vec<&(usize, usize)>,
    gap_y: &HashSet<usize>,
    gap_x: &HashSet<usize>,
    gap_size: usize,
) -> usize {
    let ((fx, fy), (tx, ty)) = (v[0], v[1]);

    let mut gaps = 0;
    fn gap_count(a: usize, b: usize, gap: &HashSet<usize>) -> usize {
        match a.cmp(&b) {
            Ordering::Less => gap.iter().filter(|&x| *x > a && *x < b).count(),
            Ordering::Greater => gap.iter().filter(|&x| *x > b && *x < a).count(),
            _ => 0,
        }
    }
    gaps += gap_count(*fx, *tx, gap_x);
    gaps += gap_count(*fy, *ty, gap_y);

    (gaps * gap_size) + fx.abs_diff(*tx) + fy.abs_diff(*ty) - gaps
}

#[cfg(test)]
mod tests {
    use crate::year_2023::{day_11::v1::*, Data};

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_11_part_1")
            .expect("src/year_2023/day_11_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_1(input), Some(9_599_070));
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_11_part_2")
            .expect("src/year_2023/day_11_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_2(input), Some(842_645_913_794));
    }
}
