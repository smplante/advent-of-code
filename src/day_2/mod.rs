mod inputs;

pub fn run() {
    println!("---------------------");
    println!("Advent of Code Day 02");
    println!("---------------------\n");

    let result_test_first = calculate_strategy(process_input_first(inputs::test));
    let result_test_second = calculate_strategy(process_input_second(inputs::test));
    println!(
        "test:   first half: {}, second half: {}",
        result_test_first, result_test_second
    );
    let result_actual_first = calculate_strategy(process_input_first(inputs::actual));
    let result_actual_second = calculate_strategy(process_input_second(inputs::actual));
    println!(
        "actual: first half: {}, second half: {}",
        result_actual_first, result_actual_second
    );
    println!("");
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

fn process_input_first(raw: &str) -> Vec<(Move, Move)> {
    raw.split("\n")
        .into_iter()
        .map(|matchup_raw| {
            let matchup: Vec<&str> = matchup_raw.split_whitespace().collect();
            (letter_to_move(matchup[0]), letter_to_move(matchup[1]))
        })
        .collect()
}

fn process_input_second(raw: &str) -> Vec<(Move, Move)> {
    raw.split("\n")
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
