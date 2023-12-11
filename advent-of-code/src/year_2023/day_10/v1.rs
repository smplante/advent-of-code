use itertools::Itertools;

pub fn part_1(input: &str) -> Option<u32> {
    let map = input.lines().map(str::as_bytes).collect::<Vec<&[u8]>>();

    let (mut pos_x, mut pos_y) = map
        .iter()
        .enumerate()
        .find_map(|(y, &xs)| Some((xs.iter().position(|x| x == &b'S')?, y)))?;

    let (start_x, start_y) = (pos_x, pos_y);
    let mut dir = Dir::Down;
    let mut steps = 0;

    if let Some((d, (x, y))) = walk(
        &Dir::Down,
        (start_x, start_y + 1),
        map[start_y + 1][start_x],
    ) {
        steps += 2;
        (pos_x, pos_y) = (x, y);
        dir = d;
    } else if let Some((d, (x, y))) =
        walk(&Dir::Up, (start_x, start_y - 1), map[start_y - 1][start_x])
    {
        steps += 2;
        (pos_x, pos_y) = (x, y);
        dir = d;
    } else if let Some((d, (x, y))) = walk(
        &Dir::Right,
        (start_x + 1, start_y),
        map[start_y][start_x + 1],
    ) {
        steps += 2;
        (pos_x, pos_y) = (x, y);
        dir = d;
    } else if let Some((d, (x, y))) = walk(
        &Dir::Left,
        (start_x - 1, start_y),
        map[start_y][start_x - 1],
    ) {
        steps += 2;
        (pos_x, pos_y) = (x, y);
        dir = d;
    }

    loop {
        match walk(&dir, (pos_x, pos_y), map[pos_y][pos_x]) {
            None => break,
            Some((d, (x, y))) => {
                dir = d;
                (pos_x, pos_y) = (x, y);
                steps += 1;
            }
        }
    }

    Some(steps / 2)
}

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn walk(dir: &Dir, pos: (usize, usize), fence: u8) -> Option<(Dir, (usize, usize))> {
    let (pos_x, pos_y) = pos;
    match (fence, dir) {
        (b'L', Dir::Down) | (b'F', Dir::Up) | (b'-', Dir::Right) => {
            Some((Dir::Right, (pos_x + 1, pos_y)))
        }
        (b'7', Dir::Up) | (b'-', Dir::Left) | (b'J', Dir::Down) => {
            Some((Dir::Left, (pos_x - 1, pos_y)))
        }
        (b'|', Dir::Up) | (b'L', Dir::Left) | (b'J', Dir::Right) => {
            Some((Dir::Up, (pos_x, pos_y - 1)))
        }
        (b'F', Dir::Left) | (b'7', Dir::Right) | (b'|', Dir::Down) => {
            Some((Dir::Down, (pos_x, pos_y + 1)))
        }
        _ => None,
    }
}

#[allow(dead_code)]
pub fn part_2(input: &str) -> Option<usize> {
    let map = input
        .lines()
        .map(str::as_bytes)
        .map(|b| b.to_owned())
        .collect::<Vec<Vec<u8>>>();
    let max_y = map.len();
    let max_x = map[0].len();

    let mut fence_map = vec![vec![b'.'; max_x * 2]; max_y * 2];

    let (mut pos_x, mut pos_y) = map
        .iter()
        .enumerate()
        .find_map(|(y, xs)| Some((xs.iter().position(|x| x == &b'S')?, y)))?;

    let (start_x, start_y) = (pos_x, pos_y);
    let start_dir = Dir::Down;
    let mut dir = Dir::Down;

    if let Some((d, (x, y))) = walk(
        &Dir::Down,
        (start_x, start_y + 1),
        map[start_y + 1][start_x],
    ) {
        fence_map[2 * pos_y + 1][2 * pos_x] = b'|';
        fence_map[2 * (pos_y + 1)][2 * pos_x] = map[pos_y + 1][pos_x];
        (pos_x, pos_y) = (x, y);
        dir = d;
    } else if let Some((d, (x, y))) =
        walk(&Dir::Up, (start_x, start_y - 1), map[start_y - 1][start_x])
    {
        fence_map[2 * pos_y - 1][2 * pos_x] = b'|';
        fence_map[2 * (pos_y - 1)][2 * pos_x] = map[pos_y - 1][pos_x];
        (pos_x, pos_y) = (x, y);
        dir = d;
    } else if let Some((d, (x, y))) = walk(
        &Dir::Right,
        (start_x + 1, start_y),
        map[start_y][start_x + 1],
    ) {
        fence_map[2 * pos_y][2 * pos_x + 1] = b'-';
        fence_map[2 * pos_y][2 * (pos_x + 1)] = map[pos_y][pos_x + 1];
        (pos_x, pos_y) = (x, y);
        dir = d;
    } else if let Some((d, (x, y))) = walk(
        &Dir::Left,
        (start_x - 1, start_y),
        map[start_y][start_x - 1],
    ) {
        fence_map[2 * pos_y][2 * pos_x - 1] = b'-';
        fence_map[2 * pos_y][2 * (pos_x - 1)] = map[pos_y][pos_x - 1];
        (pos_x, pos_y) = (x, y);
        dir = d;
    }

    fence_map[2 * pos_y][2 * pos_x] = map[pos_y][pos_x];

    loop {
        match &dir {
            Dir::Up => {
                fence_map[2 * pos_y + 1][2 * pos_x] = b'|';
            }
            Dir::Down => {
                fence_map[2 * pos_y - 1][2 * pos_x] = b'|';
            }
            Dir::Left => {
                fence_map[2 * pos_y][2 * pos_x + 1] = b'-';
            }
            Dir::Right => {
                fence_map[2 * pos_y][2 * pos_x - 1] = b'-';
            }
        }
        match walk(&dir, (pos_x, pos_y), map[pos_y][pos_x]) {
            None => break,
            Some((d, (x, y))) => {
                dir = d;
                (pos_x, pos_y) = (x, y);
                fence_map[2 * pos_y][2 * pos_x] = map[pos_y][pos_x];
            }
        }
    }

    match &dir {
        Dir::Up => {
            fence_map[2 * pos_y + 1][2 * pos_x] = b'|';
        }
        Dir::Down => {
            fence_map[2 * pos_y - 1][2 * pos_x] = b'|';
        }
        Dir::Left => {
            fence_map[2 * pos_y][2 * pos_x + 1] = b'-';
        }
        Dir::Right => {
            fence_map[2 * pos_y][2 * pos_x - 1] = b'-';
        }
    }

    fence_map[2 * pos_y][2 * pos_x] = match (&start_dir, &dir) {
        (Dir::Down, Dir::Down) | (Dir::Up, Dir::Up) => b'|',
        (Dir::Left, Dir::Left) | (Dir::Right, Dir::Right) => b'-',
        (Dir::Left, Dir::Up) | (Dir::Down, Dir::Right) => b'7',
        (Dir::Left, Dir::Down) | (Dir::Up, Dir::Right) => b'J',
        (Dir::Right, Dir::Up) | (Dir::Down, Dir::Left) => b'F',
        (Dir::Right, Dir::Down) | (Dir::Up, Dir::Left) => b'L',
        _ => unreachable!(),
    };

    fence_map = fence_map
        .iter()
        .map(|row| inside_outside(row))
        .collect_vec();

    Some(
        fence_map
            .iter()
            .enumerate()
            .filter(|(y, _)| y % 2 == 0)
            .map(|(_, r)| {
                r.iter()
                    .enumerate()
                    .filter(|(x, &p)| x % 2 == 0 && p == b'.')
                    .count()
            })
            .sum(),
    )
}

fn inside_outside(row: &[u8]) -> Vec<u8> {
    let mut outside = true;
    let mut last_wall = b'|';
    row.iter()
        .map(|&p| match (outside, last_wall, p) {
            (true, _, b'.') => b' ',
            (o, _, b'|') => {
                outside = !o;
                last_wall = b'|';
                b'|'
            }
            (o, b'F', b'J') => {
                outside = !o;
                last_wall = b'J';
                b'J'
            }
            (o, b'L', b'7') => {
                outside = !o;
                last_wall = b'7';
                b'7'
            }
            (_, b'|', b'F') | (_, b'J', b'F') => {
                last_wall = b'F';
                b'F'
            }
            (_, b'F', b'7') => {
                last_wall = b'7';
                b'7'
            }
            (_, b'|', b'L') | (_, b'7', b'L') => {
                last_wall = b'L';
                b'L'
            }
            (_, b'L', b'J') => {
                last_wall = b'J';
                b'J'
            }
            (_, b'J', b'L') => {
                last_wall = b'L';
                b'L'
            }
            (_, b'7', b'F') => {
                last_wall = b'F';
                b'F'
            }
            (_, _, p) => p,
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use crate::year_2023::{day_10::v1::*, Data};

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_10_part_1")
            .expect("src/year_2023/day_10_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_1(input), Some(6786));
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_10_part_2")
            .expect("src/year_2023/day_10_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_2(input), Some(495));
    }

    #[test]
    fn in_out() {
        let test_cases = r#"..F---------7.F-7.F-7.F-7.F---7.........X  F---------7 F-7 F-7 F-7 F---7
..|.........|.|.|.|.|.|.|.|...|.........X  |.........| |.| |.| |.| |...|
..|.F-----7.|.|.|.|.|.|.|.|.F-J.........X  |.F-----7.| |.| |.| |.| |.F-J
..|.|.....|.|.|.|.|.|.|.|.|.|...........X  |.|     |.| |.| |.| |.| |.|
..|.|...F-J.|.|.|.|.|.|.|.|.L-7.........X  |.|   F-J.| |.| |.| |.| |.L-7
..|.|...|...|.|.|.|.|.|.|.|...|.........X  |.|   |...| |.| |.| |.| |...|
F-J.L-7.L-7.L-J.L-J.|.|.L-J...L---7.....XF-J.L-7 L-7.L-J.L-J.| |.L-J...L---7
|.....|...|.........|.|...........|.....X|.....|   |.........| |...........|
L-----J...L-7.......L-J.S-7.F---7.L-7...XL-----J   L-7.......L-J.S-7.F---7.L-7
............|...........|.|.|...|...|...X            |...........| |.|   |...|   
........F---J.....F-7.F-J.|.L-7.L-7.L-7.X        F---J.....F-7.F-J |.L-7 L-7.L-7
........|.........|.|.|...|...|...|...|.X        |.........| |.|   |...|   |...|
........L-7...F-7.|.|.L-7.|...L-7.L-7.|.X        L-7...F-7.| |.L-7 |...L-7 L-7.|
..........|...|.|.|.|...|.|.....|...|.|.X          |...| |.| |...| |.....|   |.|
..........|.F-J.L-J.|.F-J.|.F-7.|...L-J.X          |.F-J L-J |.F-J |.F-7.|   L-J
..........|.|.......|.|...|.|.|.|.......X          |.|       |.|   |.| |.|
........F-J.L---7...|.|...|.|.|.|.......X        F-J.L---7   |.|   |.| |.|
........|.......|...|.|...|.|.|.|.......X        |.......|   |.|   |.| |.|
........L-------J...L-J...L-J.L-J.......X        L-------J   L-J   L-J L-J
........................................X                                        "#;

        test_cases
            .lines()
            .map(|tc| tc.split_once('X').unwrap())
            .for_each(|(i, o)| {
                let p = inside_outside(i.as_bytes());
                assert_eq!(std::str::from_utf8(&p).unwrap().trim(), o.trim());
            });
    }
}
