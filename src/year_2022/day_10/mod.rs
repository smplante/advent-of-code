mod inputs;

pub fn run() {
    let start = std::time::SystemTime::now();
    println!("--------------------------");
    println!("Advent of Code 2022 Day 10");
    println!("--------------------------\n");

    let result_test_first = process_input_first(inputs::TEST);
    println!("test:   first half: {:?}", result_test_first);

    let result_actual_first = process_input_first(inputs::ACTUAL);
    println!("actual: first half: {:?}\n", result_actual_first);
    println!("Day 10 completed in: {:?}\n", start.elapsed().unwrap());
}

#[derive(Debug)]
enum Command {
    Noop,
    Addx(i32),
}

fn process_input_first(raw: &str) -> usize {
    let (_, _, signal_strength, crt_row) = raw
        .split("\n")
        .map(|line| {
            if line.starts_with("noop") {
                Command::Noop
            } else {
                Command::Addx(line.split_once(" ").unwrap().1.parse::<i32>().unwrap())
            }
        })
        .fold(
            (1, 1, 0, ['.'; 40]),
            |(x, cycle, signal_strength, crt_row), command| {
                let mut crt_row = crt_row;
                let crt_position = (cycle - 1) % 40;
                if crt_position == 0 {
                    crt_row.iter().for_each(|c| {
                        print!("{}", c);
                    });
                    println!();
                    crt_row = ['.'; 40];
                }
                if crt_position == x - 1 || crt_position == x || crt_position == x + 1 {
                    crt_row[crt_position] = '#';
                }

                let mut signal_strength_local = 0;
                if (cycle - 20) % 40 == 0 {
                    signal_strength_local += cycle * x;
                }
                match command {
                    Command::Noop => (
                        x,
                        cycle + 1,
                        signal_strength + signal_strength_local,
                        crt_row,
                    ),
                    Command::Addx(a) => {
                        let cycle = cycle + 1;
                        let crt_position = (cycle - 1) % 40;
                        if crt_position == 0 {
                            crt_row.iter().for_each(|c| {
                                print!("{}", c);
                            });
                            println!();
                            crt_row = ['.'; 40];
                        }
                        if crt_position == x - 1 || crt_position == x || crt_position == x + 1 {
                            crt_row[crt_position] = '#';
                        }
                        if (cycle - 20) % 40 == 0 {
                            signal_strength_local += cycle * x;
                        }
                        (
                            x + (a as usize),
                            cycle + 1,
                            signal_strength + signal_strength_local,
                            crt_row,
                        )
                    }
                }
            },
        );

    crt_row.iter().for_each(|c| {
        print!("{}", c);
    });
    println!();

    signal_strength
}
