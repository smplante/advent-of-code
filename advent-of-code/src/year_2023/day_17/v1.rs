use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};
use std::ops::Add;

pub fn part_1(input: &str) -> Option<usize> {
    let grid = input
        .lines()
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|c| (c - b'0') as usize)
                .collect_vec()
        })
        .collect_vec();

    let mut pq = BinaryHeap::new();
    pq.push(Position {
        heat: 0,
        x: 0,
        y: 0,
        dir: Dir::X,
        steps: 0,
    });

    let mut visited = vec![vec![None; grid[0].len()]; grid.len()];

    let mut prev_heat = BTreeMap::new();

    while !pq.is_empty() {
        let cur = pq.pop().unwrap();
        if let Some((d, s, h)) = visited[cur.y][cur.x] {
            if d == cur.dir && s == cur.steps {
                continue;
            }
            if h > cur.heat {
                visited[cur.y][cur.x] = Some((cur.dir, cur.steps, cur.heat));
            }
        } else {
            visited[cur.y][cur.x] = Some((cur.dir, cur.steps, cur.heat));
        }
        prev_heat.insert((cur.x, cur.y, cur.dir, cur.steps), cur.heat);

        if cur.x == grid[0].len() - 1 && cur.y == grid.len() - 1 {
            break;
        }

        for d in [Dir::E, Dir::S, Dir::W, Dir::N]
            .iter()
            .filter(|d| match (cur.dir, d) {
                (Dir::N, Dir::S) | (Dir::S, Dir::N) | (Dir::E, Dir::W) | (Dir::W, Dir::E) => false,
                _ => true,
            })
            .filter(|d| {
                if **d == cur.dir && cur.steps >= 2 {
                    return false;
                }
                true
            })
        {
            let mut next_pos = match d {
                Dir::E if cur.x < grid[0].len() - 1 => Position {
                    heat: grid[cur.y][cur.x + 1],
                    x: cur.x + 1,
                    y: cur.y,
                    dir: Dir::E,
                    steps: 0,
                },
                Dir::W if cur.x > 1 => Position {
                    heat: grid[cur.y][cur.x - 1],
                    x: cur.x - 1,
                    y: cur.y,
                    dir: Dir::W,
                    steps: 0,
                },
                Dir::S if cur.y < grid.len() - 1 => Position {
                    heat: grid[cur.y + 1][cur.x],
                    x: cur.x,
                    y: cur.y + 1,
                    dir: Dir::S,
                    steps: 0,
                },
                Dir::N if cur.y > 0 => Position {
                    heat: grid[cur.y - 1][cur.x],
                    x: cur.x,
                    y: cur.y - 1,
                    dir: Dir::N,
                    steps: 0,
                },
                _ => {
                    continue;
                }
            };
            if cur.dir == next_pos.dir {
                next_pos.steps = cur.steps + 1;
            }
            if prev_heat.contains_key(&(next_pos.x, next_pos.y, next_pos.dir, next_pos.steps)) {
                continue;
            }
            let new_heat_loss = cur.heat + next_pos.heat;
            next_pos.heat = new_heat_loss;

            pq.push(next_pos);
        }
    }

    Some(visited[grid.len() - 1][grid[0].len() - 1].unwrap().2)
}

pub fn part_2(input: &str) -> Option<usize> {
    let grid = input
        .lines()
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|c| (c - b'0') as usize)
                .collect_vec()
        })
        .collect_vec();

    let mut pq = BinaryHeap::new();
    pq.push(Position {
        heat: grid[0][1] + grid[0][2] + grid[0][3] + grid[0][4],
        x: 4,
        y: 0,
        dir: Dir::E,
        steps: 3,
    });
    pq.push(Position {
        heat: grid[1][0] + grid[2][0] + grid[3][0] + grid[4][0],
        x: 0,
        y: 4,
        dir: Dir::S,
        steps: 3,
    });

    let mut visited = vec![vec![None; grid[0].len()]; grid.len()];

    let mut prev_heat = BTreeMap::new();

    let mut cycles = 0;
    while !pq.is_empty() {
        cycles += 1;
        let cur = pq.pop().unwrap();
        if let Some((d, s, h)) = visited[cur.y][cur.x] {
            if d == cur.dir && s == cur.steps {
                continue;
            }
            if h > cur.heat {
                visited[cur.y][cur.x] = Some((cur.dir, cur.steps, cur.heat));
            }
        } else {
            visited[cur.y][cur.x] = Some((cur.dir, cur.steps, cur.heat));
        }
        prev_heat.insert((cur.x, cur.y, cur.dir, cur.steps), cur.heat);

        if cur.x == grid[0].len() - 1 && cur.y == grid.len() - 1 {
            break;
        }

        for d in [Dir::E, Dir::S, Dir::W, Dir::N]
            .iter()
            .filter(|d| match (cur.dir, d) {
                (Dir::N, Dir::S) | (Dir::S, Dir::N) | (Dir::E, Dir::W) | (Dir::W, Dir::E) => false,
                _ => true,
            })
            .filter(|d| {
                if **d == cur.dir && cur.steps >= 9 {
                    return false;
                }
                if **d != cur.dir && cur.steps < 3 {
                    return false;
                }
                if **d != cur.dir && cur.steps < 3 {
                    return false;
                }
                true
            })
        {
            let mut next_pos = match (cur.dir, cur.steps, d) {
                (Dir::E, s, Dir::E) if s >= 3 && cur.x < grid[0].len() - 1 => Position {
                    heat: grid[cur.y][cur.x + 1],
                    x: cur.x + 1,
                    y: cur.y,
                    dir: Dir::E,
                    steps: s + 1,
                },
                (Dir::W, s, Dir::W) if s >= 3 && cur.x > 1 => Position {
                    heat: grid[cur.y][cur.x - 1],
                    x: cur.x - 1,
                    y: cur.y,
                    dir: Dir::W,
                    steps: s + 1,
                },
                (Dir::S, s, Dir::S) if s >= 3 && cur.y < grid.len() - 1 => Position {
                    heat: grid[cur.y + 1][cur.x],
                    x: cur.x,
                    y: cur.y + 1,
                    dir: Dir::S,
                    steps: s + 1,
                },
                (Dir::N, s, Dir::N) if s >= 3 && cur.y > 0 => Position {
                    heat: grid[cur.y - 1][cur.x],
                    x: cur.x,
                    y: cur.y - 1,
                    dir: Dir::N,
                    steps: s + 1,
                },
                (Dir::N | Dir::S, _, Dir::E) if cur.x + 4 < grid[0].len() => Position {
                    heat: grid[cur.y][cur.x + 1]
                        + grid[cur.y][cur.x + 2]
                        + grid[cur.y][cur.x + 3]
                        + grid[cur.y][cur.x + 4],
                    x: cur.x + 4,
                    y: cur.y,
                    dir: Dir::E,
                    steps: 3,
                },
                (Dir::N | Dir::S, _, Dir::W) if cur.x > 3 => Position {
                    heat: grid[cur.y][cur.x - 1]
                        + grid[cur.y][cur.x - 2]
                        + grid[cur.y][cur.x - 3]
                        + grid[cur.y][cur.x - 4],
                    x: cur.x - 4,
                    y: cur.y,
                    dir: Dir::W,
                    steps: 3,
                },
                (Dir::E | Dir::W, _, Dir::S) if cur.y + 4 < grid.len() => Position {
                    heat: grid[cur.y + 1][cur.x]
                        + grid[cur.y + 2][cur.x]
                        + grid[cur.y + 3][cur.x]
                        + grid[cur.y + 4][cur.x],
                    x: cur.x,
                    y: cur.y + 4,
                    dir: Dir::S,
                    steps: 3,
                },
                (Dir::E | Dir::W, _, Dir::N) if cur.y > 3 => Position {
                    heat: grid[cur.y - 1][cur.x]
                        + grid[cur.y - 2][cur.x]
                        + grid[cur.y - 3][cur.x]
                        + grid[cur.y - 4][cur.x],
                    x: cur.x,
                    y: cur.y - 4,
                    dir: Dir::N,
                    steps: 3,
                },
                _ => {
                    continue;
                }
            };

            if prev_heat.contains_key(&(next_pos.x, next_pos.y, next_pos.dir, next_pos.steps)) {
                continue;
            }
            let new_heat_loss = cur.heat + next_pos.heat;
            next_pos.heat = new_heat_loss;

            pq.push(next_pos);
        }
    }

    Some(visited[grid.len() - 1][grid[0].len() - 1].unwrap().2)
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
enum Dir {
    N,
    S,
    E,
    W,
    X,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Position {
    heat: usize,
    x: usize,
    y: usize,
    dir: Dir,
    steps: usize,
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat.cmp(&self.heat)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::year_2023::Data;

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_17_part_1")
            .expect("src/year_2023/day_17_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_1(input), Some(916));
    }

    #[test]
    fn debug_1() {
        assert_eq!(
            part_1(
                r"1111
1111
1111
1111"
            ),
            Some(6)
        );
    }

    #[test]
    fn debug_2() {
        assert_eq!(
            part_1(
                r"1111
2222
3333
4444"
            ),
            Some(12)
        );
    }

    #[test]
    fn debug_3() {
        assert_eq!(
            part_1(
                r"14999
23111
99991"
            ),
            Some(11)
        );
    }

    #[test]
    fn debug_4() {
        assert_eq!(
            part_1(
                r"333
444
555"
            ),
            Some(15)
        );
    }

    #[test]
    fn debug_5() {
        assert_eq!(
            part_1(
                r"11111
22222
33333
44444"
            ),
            Some(14)
        );
    }

    #[test]
    fn debug_6() {
        assert_eq!(
            part_1(
                r"111111
222222
333333
444444
555555"
            ),
            Some(21)
        );
    }

    #[test]
    fn debug_7() {
        assert_eq!(
            part_1(
                r"12345
12345
12345
12345
12345"
            ),
            Some(19)
        );
    }

    #[test]
    fn debug_8() {
        assert_eq!(
            part_1(
                r"11199
12199
99199
99131
99111"
            ),
            Some(9)
        );
    }

    #[test]
    fn debug_9() {
        assert_eq!(
            part_2(
                r"111111111111
999999999991
999999999991
999999999991
999999999991"
            ),
            Some(71)
        );
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_17_part_2")
            .expect("src/year_2023/day_17_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_2(input), Some(1067));
    }
}
