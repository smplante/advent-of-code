use std::collections::HashSet;

mod inputs;

pub fn run() {
    let start = std::time::SystemTime::now();
    println!("--------------------------");
    println!("Advent of Code 2022 Day 03");
    println!("--------------------------\n");

    let result_test_first = process_input_first(inputs::TEST)
        .iter()
        .map(|chars| chars.iter().map(char_to_priority).sum::<u32>())
        .sum::<u32>();
    let result_test_second = process_input_second(inputs::TEST)
        .iter_mut()
        .map(|elves| {
            let mut common = HashSet::new();
            for &c in elves[2].iter() {
                if elves[0].contains(&c) && elves[1].contains(&c) {
                    common.insert(c);
                }
            }
            common.iter().map(char_to_priority).sum::<u32>()
        })
        .sum::<u32>();
    println!(
        "test:   first half: {:?}, second half: {:?}",
        result_test_first, result_test_second
    );

    let result_actual_first = process_input_first(inputs::ACTUAL)
        .iter()
        .map(|chars| chars.iter().map(char_to_priority).sum::<u32>())
        .sum::<u32>();
    let result_actual_second = process_input_second(inputs::ACTUAL)
        .iter_mut()
        .map(|elves| {
            let mut common = HashSet::new();
            for &c in elves[2].iter() {
                if elves[0].contains(&c) && elves[1].contains(&c) {
                    common.insert(c);
                }
            }
            common.iter().map(char_to_priority).sum::<u32>()
        })
        .sum::<u32>();
    println!(
        "actual: first half: {}, second half: {}\n",
        result_actual_first, result_actual_second
    );
    println!("Day 03 completed in: {:?}\n", start.elapsed().unwrap());
}

fn char_to_priority(c: &char) -> u32 {
    match *c as u32 {
        upper if upper >= 65 && upper <= 90 => upper - 38,
        lower if lower >= 97 && lower <= 122 => lower - 96,
        _ => 0,
    }
}

fn process_input_second(input: &str) -> Vec<[HashSet<char>; 3]> {
    let mut groups = Vec::new();
    let mut elves: [HashSet<char>; 3] = [HashSet::new(), HashSet::new(), HashSet::new()];

    input.split("\n")
        .into_iter()
        .enumerate()
        .for_each(|(pos, rucksack_raw)| {
            elves[pos % 3] = rucksack_raw.chars().collect::<HashSet<char>>();

            if pos % 3 == 2 {
                groups.push(elves.to_owned());
                elves = [HashSet::new(), HashSet::new(), HashSet::new()];
            }
        });

    groups
}

fn process_input_first(input: &str) -> Vec<HashSet<char>> {
    input.split("\n")
        .into_iter()
        .map(|rucksack_raw| {
            let (left, right) = rucksack_raw.split_at(rucksack_raw.len() / 2);

            let left_chars = left.chars().collect::<HashSet<char>>();

            right
                .chars()
                .filter(|c| left_chars.contains(c))
                .collect::<HashSet<char>>()
        })
        .collect()
}
