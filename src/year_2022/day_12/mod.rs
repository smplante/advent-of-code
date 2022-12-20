mod inputs;

pub fn run() {
    let start = std::time::SystemTime::now();
    println!("--------------------------");
    println!("Advent of Code 2022 Day 12");
    println!("--------------------------\n");

    let result_test_first = process_input_first(inputs::TEST);
    let result_test_second = process_input_second(inputs::TEST);
    println!(
        "test:     first half: {:?}, second half: {:?}",
        result_test_first, result_test_second
    );

    let result_actual_first = process_input_first(inputs::ACTUAL);
    let result_actual_second = process_input_second(inputs::ACTUAL);
    println!(
        "actual:   first half: {:?}, second half: {:?}\n",
        result_actual_first, result_actual_second
    );
    println!("Day 12 completed in: {:?}\n", start.elapsed().unwrap());
}

#[derive(Debug)]
struct Point {
    height: u8,
    steps_from_start: u16,
}

impl Point {
    fn new(height: char) -> Point {
        Point {
            height: height as u8,
            steps_from_start: u16::MAX,
        }
    }
}

fn process_input_first(raw: &str) -> u16 {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut map = raw
        .split("\n")
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = (x, y);
                        Point::new('a')
                    }
                    'E' => {
                        end = (x, y);
                        Point::new('z')
                    }
                    c => Point::new(c),
                })
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>();

    let mut walk_points: Vec<(usize, usize, u16)> = vec![(start.0, start.1, 0)];

    while walk_points.len() > 0 {
        let (x, y, steps_from_start) = walk_points.remove(0);

        let steps_from_start = steps_from_start + 1;

        for (to_x, to_y) in [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)] {
            if to_x >= map[0].len() || to_y >= map.len() {
                continue;
            }

            if map[to_y][to_x].height > map[y][x].height + 1 {
                continue;
            }

            if map[to_y][to_x].height == map[y][x].height + 1
            && map[to_y][to_x].steps_from_start > steps_from_start {
                map[to_y][to_x].steps_from_start = steps_from_start;
                walk_points.push((to_x,to_y,steps_from_start));
                continue;
            }

            if map[to_y][to_x].height <= map[y][x].height
            && map[to_y][to_x].steps_from_start > steps_from_start {
                map[to_y][to_x].steps_from_start = steps_from_start;
                walk_points.push((to_x,to_y,steps_from_start));
                continue;
            }

        }
    }
    
    map[end.1][end.0].steps_from_start
}

fn process_input_second(raw: &str) -> u16 {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut map = raw
        .split("\n")
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = (x, y);
                        Point::new('a')
                    }
                    'E' => {
                        end = (x, y);
                        Point::new('z')
                    }
                    c => Point::new(c),
                })
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>();

    let mut walk_points: Vec<(usize, usize, u16)> = vec![(end.0, end.1, 0)];

    while walk_points.len() > 0 {
        let (x, y, steps_from_start) = walk_points.remove(0);

        let steps_from_start = steps_from_start + 1;

        for (to_x, to_y) in [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)] {
            if to_x >= map[0].len() || to_y >= map.len() {
                continue;
            }

            if map[y][x].height > map[to_y][to_x].height + 1 {
                continue;
            }

            if map[y][x].height == map[to_y][to_x].height + 1
            && map[to_y][to_x].steps_from_start > steps_from_start {
                if map[to_y][to_x].height == 'a' as u8 {
                    return steps_from_start
                }
                map[to_y][to_x].steps_from_start = steps_from_start;
                walk_points.push((to_x,to_y,steps_from_start));
                continue;
            }

            if map[y][x].height <= map[to_y][to_x].height
            && map[to_y][to_x].steps_from_start > steps_from_start {
                if map[to_y][to_x].height == 'a' as u8 {
                    return steps_from_start
                }
                map[to_y][to_x].steps_from_start = steps_from_start;
                walk_points.push((to_x,to_y,steps_from_start));
                continue;
            }
        }
    }

    u16::MAX
}
