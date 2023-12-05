use std::cmp::{max, min};

mod inputs;

pub fn run() {
    let start = std::time::SystemTime::now();
    println!("--------------------------");
    println!("Advent of Code 2022 Day 14");
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
    println!("Day 14 completed in: {:?}\n", start.elapsed().unwrap());
}

fn process_input_first(input: &str) -> usize {
    let window_size = 2;

    let mut cave = vec![vec!['.'; 1000]; 200];

    cave[0][500] = '+';

    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;

    input.split('\n').for_each(|line| {
        line.split(" -> ")
            .map(|coord| {
                let coord = coord.split_once(',').unwrap();

                (
                    coord.0.parse::<usize>().unwrap(),
                    coord.1.parse::<usize>().unwrap(),
                )
            })
            .collect::<Vec<(usize, usize)>>()
            .windows(window_size)
            .for_each(|window| {
                let (from_x, from_y) = window[0];
                let (to_x, to_y) = window[1];

                if min(from_x, to_x) <= min_x {
                    min_x = min(from_x, to_x) - 2
                }
                if max(from_x, to_x) >= max_x {
                    max_x = max(from_x, to_x) + 2
                }
                if min(from_y, to_y) >= max_y {
                    max_y = min(from_y, to_y) + 2
                }

                if from_x == to_x {
                    if from_y < to_y {
                        for y in from_y..=to_y {
                            cave[y][from_x] = '#';
                        }
                    } else {
                        for y in to_y..=from_y {
                            cave[y][from_x] = '#';
                        }
                    }
                }
                if from_y == to_y {
                    if from_x < to_x {
                        for x in from_x..=to_x {
                            cave[from_y][x] = '#';
                        }
                    } else {
                        for x in to_x..=from_x {
                            cave[from_y][x] = '#';
                        }
                    }
                }
            })
    });

    let mut sand_count = 0;
    loop {
        sand_count += 1;
        let mut sand = (500usize, 0usize);
        loop {
            let (x, y) = (sand.0, sand.1 + 1);
            if y > max_y {
                return sand_count - 1;
            }

            if cave[y][x] == '#' || cave[y][x] == 'o' {
                if cave[y][x - 1] == '#' || cave[y][x - 1] == 'o' {
                    if cave[y][x + 1] == '#' || cave[y][x + 1] == 'o' {
                        cave[sand.1][sand.0] = 'o';
                        break;
                    }
                    sand = (x + 1, y);
                    continue;
                }
                sand = (x - 1, y);
                continue;
            }
            sand = (x, y);
        }
    }
}

fn process_input_second(input: &str) -> usize {
    let window_size = 2;

    let mut cave = vec![vec!['.'; 1000]; 200];

    cave[0][500] = '+';

    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;

    input.split('\n').for_each(|line| {
        line.split(" -> ")
            .map(|coord| {
                let coord = coord.split_once(',').unwrap();

                (
                    coord.0.parse::<usize>().unwrap(),
                    coord.1.parse::<usize>().unwrap(),
                )
            })
            .collect::<Vec<(usize, usize)>>()
            .windows(window_size)
            .for_each(|window| {
                let (from_x, from_y) = window[0];
                let (to_x, to_y) = window[1];

                if min(from_x, to_x) <= min_x {
                    min_x = min(from_x, to_x) - 2
                }
                if max(from_x, to_x) >= max_x {
                    max_x = max(from_x, to_x) + 2
                }
                if min(from_y, to_y) >= max_y {
                    max_y = min(from_y, to_y) + 2
                }

                if from_x == to_x {
                    if from_y < to_y {
                        for y in from_y..=to_y {
                            cave[y][from_x] = '#';
                        }
                    } else {
                        for y in to_y..=from_y {
                            cave[y][from_x] = '#';
                        }
                    }
                }
                if from_y == to_y {
                    if from_x < to_x {
                        for x in from_x..=to_x {
                            cave[from_y][x] = '#';
                        }
                    } else {
                        for x in to_x..=from_x {
                            cave[from_y][x] = '#';
                        }
                    }
                }
            })
    });

    for c in 0..(cave[0].len()) {
        cave[max_y][c] = '#';
    }

    let mut sand_count = 0;
    loop {
        sand_count += 1;
        let mut sand = (500usize, 0usize);
        loop {
            let (x, y) = (sand.0, sand.1 + 1);

            if cave[y][x] == '#' || cave[y][x] == 'o' {
                if cave[y][x - 1] == '#' || cave[y][x - 1] == 'o' {
                    if cave[y][x + 1] == '#' || cave[y][x + 1] == 'o' {
                        if sand.0 == 500 && sand.1 == 0 {
                            return sand_count;
                        }
                        cave[sand.1][sand.0] = 'o';
                        break;
                    }
                    sand = (x + 1, y);
                    continue;
                }
                sand = (x - 1, y);
                continue;
            }
            sand = (x, y);
        }
    }
}

// #[cfg(test)]
// mod tests {
//     extern crate test;
//
//     use super::*;
//     use test::Bencher;
//
//     #[bench]
//     fn bench_process_input_first(b: &mut Bencher) {
//         b.iter(|| process_input_first(inputs::ACTUAL));
//     }
//
//     #[bench]
//     fn bench_process_input_second(b: &mut Bencher) {
//         b.iter(|| process_input_second(inputs::ACTUAL));
//     }
// }
