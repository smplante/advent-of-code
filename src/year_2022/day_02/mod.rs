mod inputs;

pub fn run() {
    let start = std::time::SystemTime::now();
    println!("--------------------------");
    println!("Advent of Code 2022 Day 02");
    println!("--------------------------\n");

    let result_test_first = calculate_strategy(process_input_first(inputs::TEST));
    let result_test_second = calculate_strategy(process_input_second(inputs::TEST));
    println!(
        "test:   first half: {}, second half: {}",
        result_test_first, result_test_second
    );
    let result_actual_first = calculate_strategy(process_input_first(inputs::ACTUAL));
    let result_actual_second = calculate_strategy(process_input_second(inputs::ACTUAL));
    println!(
        "actual: first half: {}, second half: {}\n",
        result_actual_first, result_actual_second
    );
    println!("Day 02 completed in: {:?}\n", start.elapsed().unwrap());
}

enum Move {
    Rock(u32),
    Paper(u32),
    Scissors(u32),
}

fn letter_to_move(letter: &str) -> Move {
    match letter {
        "A" | "X" => Move::Rock(1),
        "B" | "Y" => Move::Paper(2),
        "C" | "Z" => Move::Scissors(3),
        _ => panic!("this shouldn't happen"),
    }
}

fn letter_to_target_move(opponent: &str, target: &str) -> Move {
    match (opponent, target) {
        ("A", "X") => Move::Scissors(3),
        ("A", "Y") => Move::Rock(1),
        ("A", "Z") => Move::Paper(2),
        ("B", "X") => Move::Rock(1),
        ("B", "Y") => Move::Paper(2),
        ("B", "Z") => Move::Scissors(3),
        ("C", "X") => Move::Paper(2),
        ("C", "Y") => Move::Scissors(3),
        ("C", "Z") => Move::Rock(1),
        _ => panic!("this shouldn't happen"),
    }
}

fn process_input_first(input: &str) -> Vec<(Move, Move)> {
    input
        .split("\n")
        .into_iter()
        .map(|matchup_raw| {
            let matchup: Vec<&str> = matchup_raw.split_whitespace().collect();
            (letter_to_move(matchup[0]), letter_to_move(matchup[1]))
        })
        .collect()
}

fn process_input_second(input: &str) -> Vec<(Move, Move)> {
    input
        .split("\n")
        .into_iter()
        .map(|matchup_raw| {
            let matchup: Vec<&str> = matchup_raw.split_whitespace().collect();
            (
                letter_to_move(matchup[0]),
                letter_to_target_move(matchup[0], matchup[1]),
            )
        })
        .collect()
}

fn calculate_strategy(matchups: Vec<(Move, Move)>) -> u32 {
    matchups
        .iter()
        .map(|matchup| match (&matchup.0, &matchup.1) {
            (Move::Rock(_), Move::Rock(b)) => b + 3,
            (Move::Rock(_), Move::Paper(b)) => b + 6,
            (Move::Rock(_), Move::Scissors(b)) => b + 0,
            (Move::Paper(_), Move::Rock(b)) => b + 0,
            (Move::Paper(_), Move::Paper(b)) => b + 3,
            (Move::Paper(_), Move::Scissors(b)) => b + 6,
            (Move::Scissors(_), Move::Rock(b)) => b + 6,
            (Move::Scissors(_), Move::Paper(b)) => b + 0,
            (Move::Scissors(_), Move::Scissors(b)) => b + 3,
        })
        .sum::<u32>()
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
//         b.iter(|| calculate_strategy(process_input_first(inputs::ACTUAL)));
//     }
//
//     #[bench]
//     fn bench_process_input_second(b: &mut Bencher) {
//         b.iter(|| calculate_strategy(process_input_second(inputs::ACTUAL)));
//     }
// }
