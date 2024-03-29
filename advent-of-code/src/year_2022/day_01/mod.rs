mod inputs;

pub fn run() {
    println!("--------------------------");
    println!("Advent of Code 2022 Day 01");
    println!("--------------------------\n");

    let start = std::time::SystemTime::now();
    let result_test = collect_calories(process_input(inputs::TEST));
    println!(
        "test:   first half: {}, second half: {}",
        result_test.0,
        result_test.0 + result_test.1 + result_test.2
    );
    let result_actual = collect_calories(process_input(inputs::ACTUAL));
    println!(
        "actual: first half: {}, second half: {}\n",
        result_actual.0,
        result_actual.0 + result_actual.1 + result_actual.2
    );
    println!("Day 01 completed in: {:?}\n", start.elapsed().unwrap());
}

fn process_input(input: &str) -> Vec<&str> {
    let split = input.split('\n');
    let output: Vec<&str> = split.collect();

    output
}

fn collect_calories(list: Vec<&str>) -> (u32, u32, u32) {
    let mut elves = vec![];
    let mut elf = 0;
    let mut top_1 = 0;
    let mut top_2 = 0;
    let mut top_3 = 0;

    for &raw_calories in list.iter() {
        if raw_calories.is_empty() {
            if elves[elf] >= top_1 {
                top_3 = top_2;
                top_2 = top_1;
                top_1 = elves[elf]
            } else if elves[elf] >= top_2 {
                top_3 = top_2;
                top_2 = elves[elf]
            } else if elves[elf] >= top_3 {
                top_3 = elves[elf]
            }
            elf += 1;
            continue;
        }

        let calories: u32 = raw_calories.parse().unwrap();
        if elves.len() == elf {
            elves.push(0);
        }
        elves[elf] += calories;
    }
    elf -= 1;
    if elves[elf] > top_1 {
        top_3 = top_2;
        top_2 = top_1;
        top_1 = elves[elf]
    } else if elves[elf] > top_2 {
        top_3 = top_2;
        top_2 = elves[elf]
    } else if elves[elf] >= top_3 {
        top_3 = elves[elf]
    }

    (top_1, top_2, top_3)
}

// #[cfg(test)]
// mod tests {
//     extern crate test;
//
//     use super::*;
//     use test::Bencher;
//
//     #[bench]
//     fn bench_process_input(b: &mut Bencher) {
//         b.iter(|| collect_calories(process_input(inputs::ACTUAL)));
//     }
//
// }
