use std::collections::HashSet;

mod inputs;

pub fn run() {
    let start = std::time::SystemTime::now();
    println!("--------------------------");
    println!("Advent of Code 2022 Day 15");
    println!("--------------------------\n");

    let result_test_first = process_input_first(10, inputs::TEST);
    let result_test_second = process_input_second(20, inputs::TEST);
    println!(
        "test:     first half: {:?}, second half: {:?}",
        result_test_first, result_test_second
    );

    let result_actual_first = process_input_first(2000000, inputs::ACTUAL);
    let result_actual_second = process_input_second(4000000, inputs::ACTUAL);
    println!(
        "actual:   first half: {:?}, second half: {:?}\n",
        result_actual_first, result_actual_second
    );
    println!("Day 15 completed in: {:?}\n", start.elapsed().unwrap());
}

fn process_input_first(target: i64, input: &str) -> i64 {
    let mut beacons = HashSet::new();
    let mut ranges = input
        .split("\n")
        .map(|line| {
            let sensor = &line[(line.find("x").unwrap())..(line.find(":").unwrap())];
            let sensor_x = sensor[(sensor.find("x").unwrap() + 2)..(sensor.find(",").unwrap())]
                .parse::<i64>()
                .unwrap();
            let sensor_y = sensor[(sensor.find("y").unwrap() + 2)..]
                .parse::<i64>()
                .unwrap();

            let beacon = &line[(line.find(":").unwrap())..];
            let beacon_x = beacon[(beacon.find("x").unwrap() + 2)..(beacon.find(",").unwrap())]
                .parse::<i64>()
                .unwrap();
            let beacon_y = beacon[(beacon.find("y").unwrap() + 2)..]
                .parse::<i64>()
                .unwrap();

            if beacon_y == target {
                beacons.insert((beacon_x, beacon_y));
            }

            let radius = i64::abs(sensor_x - beacon_x) + i64::abs(sensor_y - beacon_y);

            (sensor_x, sensor_y, radius)
        })
        .filter(|(_, y, r)| i64::abs(y - target) <= *r)
        .map(|(x, y, r)| {
            let width = 1 + 2 * (r - i64::abs(y - target));
            let (start, end) = (x - (width / 2), x + (width / 2));

            (start, end)
        })
        .collect::<Vec<(i64, i64)>>();

    ranges.sort_by(|left, right| {
        let mut order = left.0.cmp(&right.0);
        if order == std::cmp::Ordering::Equal {
            order = left.1.cmp(&right.1);
        }
        order
    });

    let mut count = 0;

    let mut start = ranges[0].0;
    let mut end = ranges[0].0;

    ranges.iter().for_each(|&(s, e)| {
        if s <= end && e > end {
            end = e;
            return;
        }
        if s > end {
            let b = beacons
                .iter()
                .filter(|&b| b.0 >= start && b.0 <= end)
                .count();
            count += 1 + (end - start) - (b as i64);
            start = s;
            end = e;
        }
    });

    let b = beacons
        .iter()
        .filter(|&b| b.0 >= start && b.0 <= end)
        .count();

    count + (1 + end - start) - (b as i64)
}

fn process_input_second(target: i64, input: &str) -> i64 {
    let ranges = input
        .split("\n")
        .map(|line| {
            let sensor = &line[(line.find("x").unwrap())..(line.find(":").unwrap())];
            let sensor_x = sensor[(sensor.find("x").unwrap() + 2)..(sensor.find(",").unwrap())]
                .parse::<i64>()
                .unwrap();
            let sensor_y = sensor[(sensor.find("y").unwrap() + 2)..]
                .parse::<i64>()
                .unwrap();

            let beacon = &line[(line.find(":").unwrap())..];
            let beacon_x = beacon[(beacon.find("x").unwrap() + 2)..(beacon.find(",").unwrap())]
                .parse::<i64>()
                .unwrap();
            let beacon_y = beacon[(beacon.find("y").unwrap() + 2)..]
                .parse::<i64>()
                .unwrap();

            let radius = i64::abs(sensor_x - beacon_x) + i64::abs(sensor_y - beacon_y);

            (sensor_x, sensor_y, radius)
        })
        .collect::<Vec<(i64, i64, i64)>>();

    let mut y = 0;
    while y <= target {
        let mut ranges = ranges
            .iter()
            .map(|sensor| calculate_overlap(sensor, y, target))
            .filter(|&(s, e)| s > -1 && e > -1)
            .collect::<Vec<(i64, i64)>>();

        ranges.sort_by(|left, right| {
            let mut order = left.0.cmp(&right.0);
            if order == std::cmp::Ordering::Equal {
                order = left.1.cmp(&right.1);
            }
            order
        });

        if ranges[0].0 > 0 {
            return 0 * 4000000 + y;
        }

        let mut end = ranges[0].1;
        let mut x = -1;

        let mut skip = target;

        ranges.iter().for_each(|&(s, e)| {
            if s <= end && e > end {
                if (end - s) < skip {
                    skip = end - s;
                }
                end = e;
                return;
            }
            if s > end && end > -1 {
                x = end + 1;
            }
        });

        if x > 0 {
            return x * 4000000 + y;
        }

        y += std::cmp::max(1, skip / 2);
    }

    0
}

fn calculate_overlap((x, y, r): &(i64, i64, i64), row: i64, target: i64) -> (i64, i64) {
    if &i64::abs(y - row) > r {
        return (-1, -1);
    }
    let width = 1 + 2 * (r - i64::abs(y - row));
    let (start, end) = (
        std::cmp::max(0, x - (width / 2)),
        std::cmp::min(target, x + (width / 2)),
    );

    (start, end)
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_process_input_first(b: &mut Bencher) {
        b.iter(|| process_input_first(2000000, inputs::ACTUAL));
    }

    #[bench]
    fn bench_process_input_second(b: &mut Bencher) {
        b.iter(|| process_input_second(4000000, inputs::ACTUAL));
    }
}
