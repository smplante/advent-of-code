use std::collections::HashSet;

mod inputs;

pub fn run() {
    let start = std::time::SystemTime::now();
    println!("--------------------------");
    println!("Advent of Code 2022 Day 09");
    println!("--------------------------\n");

    let result_test_first = process_input_first(inputs::TEST);
    let result_test_second = process_input_second(inputs::TEST);
    println!(
        "test 1: first half: {:?}, second half: {:?}",
        result_test_first, result_test_second
    );

    let result_test_2_first = process_input_first(inputs::TEST_2);
    let result_test_2_second = process_input_second(inputs::TEST_2);
    println!(
        "test 2: first half: {:?}, second half: {:?}",
        result_test_2_first, result_test_2_second
    );

    let result_test_3_first = process_input_first(inputs::TEST_3);
    let result_test_3_second = process_input_second(inputs::TEST_3);
    println!(
        "test 3: first half: {:?}, second half: {:?}",
        result_test_3_first, result_test_3_second
    );

    let result_actual_first = process_input_first(inputs::ACTUAL);
    let result_actual_second = process_input_second(inputs::ACTUAL);
    println!(
        "actual: first half: {:?}, second half: {:?}\n",
        result_actual_first, result_actual_second
    );
    println!("Day 09 completed in: {:?}\n", start.elapsed().unwrap());
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Rope {
    knots: Vec<(i32, i32)>,
    tail_visits: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(size: usize) -> Rope {
        Rope {
            knots: vec![(0, 0); size],
            tail_visits: HashSet::new(),
        }
    }
    fn shift(&mut self, (direction, distance): &(Direction, i32)) {
        let change = match direction {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        for _ in 0..*distance {
            self.knots[0] = (self.knots[0].0 + change.0, self.knots[0].1 + change.1);

            for t in 1..self.knots.len() {
                let tail_change = match (
                    (self.knots[t - 1].0 - self.knots[t].0),
                    (self.knots[t - 1].1 - self.knots[t].1),
                ) {
                    (0, 0)
                    | (0, 1)
                    | (1, 0)
                    | (0, -1)
                    | (-1, 0)
                    | (1, 1)
                    | (-1, -1)
                    | (-1, 1)
                    | (1, -1) => (0, 0),
                    (2, 2) => (1, 1),
                    (2, -2) => (1, -1),
                    (-2, -2) => (-1, -1),
                    (-2, 2) => (-1, 1),
                    (2, y) => (1, y),
                    (x, 2) => (x, 1),
                    (-2, y) => (-1, y),
                    (x, -2) => (x, -1),
                    (_, _) => (0, 0),
                };

                self.knots[t] = (
                    self.knots[t].0 + tail_change.0,
                    self.knots[t].1 + tail_change.1,
                );
            }

            self.tail_visits.insert(self.knots[self.knots.len() - 1]);
        }
    }
}

fn process_input_first(input: &str) -> usize {
    let mut rope = Rope::new(2);

    input
        .split("\n")
        .map(|c| {
            let (direction, count) = c.split_once(" ").unwrap();
            match direction {
                "U" => (Direction::Up, count.parse::<i32>().unwrap()),
                "D" => (Direction::Down, count.parse::<i32>().unwrap()),
                "L" => (Direction::Left, count.parse::<i32>().unwrap()),
                "R" => (Direction::Right, count.parse::<i32>().unwrap()),
                _ => Err("invalid input").unwrap(),
            }
        })
        .for_each(|m| {
            rope.shift(&m);
        });

    rope.tail_visits.len()
}

fn process_input_second(input: &str) -> usize {
    let mut rope = Rope::new(10);

    input
        .split("\n")
        .map(|c| {
            let (direction, count) = c.split_once(" ").unwrap();
            match direction {
                "U" => (Direction::Up, count.parse::<i32>().unwrap()),
                "D" => (Direction::Down, count.parse::<i32>().unwrap()),
                "L" => (Direction::Left, count.parse::<i32>().unwrap()),
                "R" => (Direction::Right, count.parse::<i32>().unwrap()),
                _ => Err("invalid input").unwrap(),
            }
        })
        .for_each(|m| {
            rope.shift(&m);
        });

    rope.tail_visits.len()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_process_input_first(b: &mut Bencher) {
        b.iter(|| process_input_first(inputs::ACTUAL));
    }

    #[bench]
    fn bench_process_input_second(b: &mut Bencher) {
        b.iter(|| process_input_second(inputs::ACTUAL));
    }
}
