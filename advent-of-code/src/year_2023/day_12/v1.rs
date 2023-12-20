use rayon::prelude::*;
use std::collections::{BTreeMap, VecDeque};
use std::iter::once;

pub fn part_1(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .par_bridge()
            .filter_map(|l| l.split_once(' '))
            .map(|(input, output)| (input_to_deque(input), output_to_deque(output)))
            .filter_map(count_arrangements_cached)
            .sum(),
    )
}

pub fn part_2(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .par_bridge()
            .filter_map(|l| l.split_once(' '))
            .map(|(input, output)| (input_to_deque_2(input), output_to_deque_2(output)))
            .filter_map(count_arrangements_cached)
            .sum(),
    )
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Spring {
    Op,
    Dmg(usize),
    U,
}

type HashKey = (Option<Spring>, VecDeque<Spring>, VecDeque<usize>);

fn input_to_deque(input: &str) -> VecDeque<Spring> {
    let mut d = VecDeque::new();
    let mut dmg = 0;
    for b in input.as_bytes() {
        match *b {
            b'.' => {
                if dmg > 0 {
                    d.push_back(Spring::Dmg(dmg));
                    dmg = 0;
                }
                d.push_back(Spring::Op)
            }
            b'#' => {
                dmg += 1;
            }
            b'?' => {
                if dmg > 0 {
                    d.push_back(Spring::Dmg(dmg));
                    dmg = 0;
                }
                d.push_back(Spring::U)
            }
            _ => unreachable!(),
        }
    }

    if dmg > 0 {
        d.push_back(Spring::Dmg(dmg));
    }

    d
}

fn input_to_deque_2(input: &str) -> VecDeque<Spring> {
    let mut d = VecDeque::new();
    let mut dmg = 0;
    for b in input.as_bytes() {
        match *b {
            b'.' => {
                if dmg > 0 {
                    d.push_back(Spring::Dmg(dmg));
                    dmg = 0;
                }
                d.push_back(Spring::Op)
            }
            b'#' => {
                dmg += 1;
            }
            b'?' => {
                if dmg > 0 {
                    d.push_back(Spring::Dmg(dmg));
                    dmg = 0;
                }
                d.push_back(Spring::U)
            }
            _ => unreachable!(),
        }
    }

    if dmg > 0 {
        d.push_back(Spring::Dmg(dmg));
    }

    let d_clone = d.clone();

    d.into_iter()
        .chain(once(Spring::U))
        .chain(d_clone.clone())
        .chain(once(Spring::U))
        .chain(d_clone.clone())
        .chain(once(Spring::U))
        .chain(d_clone.clone())
        .chain(once(Spring::U))
        .chain(d_clone.clone())
        .collect::<VecDeque<Spring>>()
}

fn output_to_deque(output: &str) -> VecDeque<usize> {
    output
        .split(',')
        .filter_map(|o| o.parse::<usize>().ok())
        .collect::<VecDeque<_>>()
}

fn output_to_deque_2(output: &str) -> VecDeque<usize> {
    let d = output
        .split(',')
        .filter_map(|o| o.parse::<usize>().ok())
        .collect::<VecDeque<_>>();

    let d_clone = d.clone();

    d.into_iter()
        .chain(d_clone.clone())
        .chain(d_clone.clone())
        .chain(d_clone.clone())
        .chain(d_clone.clone())
        .collect()
}

fn count_arrangements_cached(io: (VecDeque<Spring>, VecDeque<usize>)) -> Option<usize> {
    let mut cache = BTreeMap::new();
    count_arrangements_rec_cached(&mut cache, vec![], io.0, io.1)
}

fn count_arrangements_rec_cached(
    cache: &mut BTreeMap<HashKey, usize>,
    p: Vec<Spring>,
    i: VecDeque<Spring>,
    o: VecDeque<usize>,
) -> Option<usize> {
    let mut in_deque = i.clone();
    let mut out_deque = o.clone();
    let prev = p.clone();

    if out_deque.is_empty() && in_deque.is_empty() {
        return Some(1);
    }
    if out_deque.is_empty() && in_deque.iter().any(|i| matches!(i, Spring::Dmg(_))) {
        return Some(0);
    }
    if out_deque.is_empty() {
        return Some(1);
    }

    if in_deque.len() + 1 < 2 * out_deque.len() - 1 {
        return Some(0);
    }

    if let Some(&c) = cache.get(&(prev.last().copied(), in_deque.clone(), out_deque.clone())) {
        return Some(c);
    }

    let m_prev = prev.last();
    let m_i = in_deque.pop_front();
    let m_ni = in_deque.get(0);
    let m_o = out_deque.get(0);
    let to_check = (m_prev, m_i, m_ni, m_o);

    match to_check {
        (Some(Spring::Dmg(pi)), Some(Spring::U) | Some(Spring::Op) | None, _, Some(e))
            if pi == e =>
        {
            let _ = out_deque.pop_front();
            let mut prev = prev.clone();
            prev.push(Spring::Op);
            let c = count_arrangements_rec_cached(
                cache,
                prev.clone(),
                in_deque.clone(),
                out_deque.clone(),
            )?;
            cache.insert((prev.last().copied(), in_deque, out_deque), c);
            Some(c)
        }
        (Some(Spring::Op), Some(Spring::Dmg(i)), None, Some(e)) if i == *e => {
            let _ = out_deque.pop_front();
            let mut prev = prev.clone();
            prev.push(Spring::Op);
            let c = count_arrangements_rec_cached(
                cache,
                prev.clone(),
                in_deque.clone(),
                out_deque.clone(),
            )?;
            cache.insert((prev.last().copied(), in_deque, out_deque), c);
            Some(c)
        }
        (Some(Spring::Dmg(pi)), Some(Spring::Dmg(i)), Some(Spring::Op) | None, Some(e))
            if pi + i == *e =>
        {
            let mut prev = prev.clone();
            let _ = prev.pop();
            prev.push(Spring::Dmg(pi + i));
            let c = count_arrangements_rec_cached(
                cache,
                prev.clone(),
                in_deque.clone(),
                out_deque.clone(),
            )?;
            cache.insert((prev.last().copied(), in_deque, out_deque), c);
            Some(c)
        }
        (Some(Spring::Op) | None, Some(Spring::U), Some(Spring::Dmg(ni)), Some(o)) if ni < o => {
            let mut p0 = prev.clone();
            p0.push(Spring::Op);
            let mut p1 = prev.clone();
            p1.push(Spring::Dmg(1));
            let c1 = count_arrangements_rec_cached(
                cache,
                p0.clone(),
                in_deque.clone(),
                out_deque.clone(),
            )?;
            cache.insert(
                (p0.last().copied(), in_deque.clone(), out_deque.clone()),
                c1,
            );
            let c2 = count_arrangements_rec_cached(
                cache,
                p1.clone(),
                in_deque.clone(),
                out_deque.clone(),
            )?;
            cache.insert((p1.last().copied(), in_deque, out_deque), c2);
            Some(c1 + c2)
        }
        (
            Some(Spring::Op) | None,
            Some(Spring::U),
            Some(Spring::Op) | Some(Spring::U) | None,
            Some(_),
        ) => {
            let mut p0 = prev.clone();
            p0.push(Spring::Op);
            let mut p1 = prev.clone();
            p1.push(Spring::Dmg(1));
            let c1 = count_arrangements_rec_cached(
                cache,
                p0.clone(),
                in_deque.clone(),
                out_deque.clone(),
            )?;
            cache.insert(
                (p0.last().copied(), in_deque.clone(), out_deque.clone()),
                c1,
            );
            let c2 = count_arrangements_rec_cached(
                cache,
                p1.clone(),
                in_deque.clone(),
                out_deque.clone(),
            )?;
            cache.insert((p1.last().copied(), in_deque, out_deque), c2);
            Some(c1 + c2)
        }
        (Some(Spring::Op) | None, Some(Spring::U), Some(Spring::Dmg(ni)), Some(o)) if ni < o => {
            let mut p0 = prev.clone();
            p0.push(Spring::Op);
            let mut p1 = prev.clone();
            p1.push(Spring::Dmg(1));
            let c1 = count_arrangements_rec_cached(
                cache,
                p0.clone(),
                in_deque.clone(),
                out_deque.clone(),
            )?;
            cache.insert(
                (p0.last().copied(), in_deque.clone(), out_deque.clone()),
                c1,
            );
            let c2 = count_arrangements_rec_cached(
                cache,
                p1.clone(),
                in_deque.clone(),
                out_deque.clone(),
            )?;
            cache.insert((p1.last().copied(), in_deque, out_deque), c2);
            Some(c1 + c2)
        }
        (Some(Spring::Op) | None, Some(Spring::U), Some(Spring::Dmg(ni)), Some(o)) if ni == o => {
            let mut prev = prev.clone();
            prev.push(Spring::Op);
            let c = count_arrangements_rec_cached(
                cache,
                prev.clone(),
                in_deque.clone(),
                out_deque.clone(),
            )?;
            cache.insert((prev.last().copied(), in_deque, out_deque), c);
            Some(c)
        }
        (
            Some(Spring::Op) | None,
            Some(Spring::Dmg(i)),
            Some(Spring::U) | Some(Spring::Op),
            Some(o),
        ) if i <= *o => {
            let mut prev = prev.clone();
            prev.push(Spring::Dmg(i));
            let c = count_arrangements_rec_cached(
                cache,
                prev.clone(),
                in_deque.clone(),
                out_deque.clone(),
            )?;
            cache.insert((prev.last().copied(), in_deque, out_deque), c);
            Some(c)
        }
        (Some(Spring::Op) | None, Some(Spring::Op), Some(_), Some(_)) => {
            let mut prev = prev.clone();
            prev.push(Spring::Op);
            let c = count_arrangements_rec_cached(
                cache,
                prev.clone(),
                in_deque.clone(),
                out_deque.clone(),
            )?;
            cache.insert((prev.last().copied(), in_deque, out_deque), c);
            Some(c)
        }
        (
            Some(Spring::Dmg(pi)),
            Some(Spring::U),
            Some(Spring::U) | Some(Spring::Op) | Some(Spring::Dmg(_)) | None,
            Some(o),
        ) if pi < o => {
            let mut prev = prev.clone();
            let _ = prev.pop();
            prev.push(Spring::Dmg(pi + 1));
            let c = count_arrangements_rec_cached(
                cache,
                prev.clone(),
                in_deque.clone(),
                out_deque.clone(),
            )?;
            cache.insert((prev.last().copied(), in_deque, out_deque), c);
            Some(c)
        }
        (
            Some(Spring::Dmg(pi)),
            Some(Spring::Dmg(i)),
            Some(Spring::U) | Some(Spring::Op) | None,
            Some(o),
        ) if pi + i <= *o => {
            let mut prev = prev.clone();
            let _ = prev.pop();
            prev.push(Spring::Dmg(pi + i));
            let c = count_arrangements_rec_cached(
                cache,
                prev.clone(),
                in_deque.clone(),
                out_deque.clone(),
            )?;
            cache.insert((prev.last().copied(), in_deque, out_deque), c);
            Some(c)
        }
        _ => Some(0),
    }
}

#[cfg(test)]
mod tests {
    use crate::year_2023::{day_12::v1::*, Data};

    #[test]
    fn count_arrangements_variable() {
        let tests = vec![
            (
                (input_to_deque(".???#?????.#?."), output_to_deque("1,4,2")),
                4,
            ),
            (
                (input_to_deque("??#??.???##?"), output_to_deque("1,1,1,5")),
                2,
            ),
            ((input_to_deque("???#.???#"), output_to_deque("1,1")), 1),
            (
                (
                    input_to_deque("?##.???#?????#.#?.??"),
                    output_to_deque("3,2,4,1,2"),
                ),
                3,
            ),
            (
                (
                    input_to_deque("#.?#??????#?#??#?##?"),
                    output_to_deque("1,1,14"),
                ),
                2,
            ),
            ((input_to_deque(".???#?????#."), output_to_deque("7,1")), 2),
            ((input_to_deque("???.###"), output_to_deque("1,1,3")), 1),
            (
                (
                    input_to_deque(".?#?..?#?...?####?..."),
                    output_to_deque("1,1,4"),
                ),
                1,
            ),
            ((input_to_deque("?????"), output_to_deque("1,1,1")), 1),
            ((input_to_deque("?????"), output_to_deque("1,2,1")), 0),
            ((input_to_deque("??????"), output_to_deque("1,2,1")), 1),
            (
                (input_to_deque("?.??.#??#?#???"), output_to_deque("1,8")),
                3,
            ),
            (
                (input_to_deque(".??..??...?##."), output_to_deque("1,1,3")),
                4,
            ),
            (
                (
                    input_to_deque("?#?#?#?#?#?#?#?"),
                    output_to_deque("1,3,1,6"),
                ),
                1,
            ),
            (
                (input_to_deque("????.#...#..."), output_to_deque("4,1,1")),
                1,
            ),
            (
                (
                    input_to_deque("????.######..#####."),
                    output_to_deque("1,6,5"),
                ),
                4,
            ),
            (
                (input_to_deque("?###????????"), output_to_deque("3,2,1")),
                10,
            ),
        ];

        for t in tests {
            println!("\n\n{:?} = {}", t.0, t.1);
            assert_eq!(count_arrangements_cached(t.0), Some(t.1));
        }
    }

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_12_part_1")
            .expect("src/year_2023/day_12_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_1(input), Some(6949));
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_12_part_2")
            .expect("src/year_2023/day_12_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_2(input), Some(51_456_609_952_403));
    }
}
