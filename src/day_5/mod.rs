mod inputs;

pub fn run() {
    println!("---------------------");
    println!("Advent of Code Day 05");
    println!("---------------------\n");

    let result_test_first = process_input_first(inputs::test);
    let result_test_second = process_input_second(inputs::test);
    println!(
        "test:   first half: {:?}, second half: {:?}",
        result_test_first, result_test_second
    );

    let result_actual_first = process_input_first(inputs::actual);
    let result_actual_second = process_input_second(inputs::actual);
    println!(
        "actual: first half: {:?}, second half: {:?}\n",
        result_actual_first, result_actual_second
    );
}

fn process_input_first(raw: &str) -> String {
    let mut stacks = Vec::new();

    let (first_half, second_half) = raw.split_once("\n\n").unwrap();

    let first_half = first_half.split("\n").collect::<Vec<&str>>();
    for _ in 0..((first_half[0].len() + 1) / 4) {
        stacks.push(Vec::new());
    }
    for line in first_half {
        let mut stack = 0;
        let chars = line.chars();

        for char in chars.skip(1).step_by(4) {
            if char == ' ' {
                stack += 1;
                continue;
            }
            stacks[stack].push(char);
            stack += 1;
        }
    }

    stacks.iter_mut().for_each(|stack| {
        stack.reverse();
    });

    second_half
        .split("\n")
        .map(|line| {
            let command_raw = line.split(" ").collect::<Vec<&str>>();
            (
                command_raw[1].parse::<usize>().unwrap(),
                command_raw[3].parse::<usize>().unwrap() - 1,
                command_raw[5].parse::<usize>().unwrap() - 1,
            )
        })
        .for_each(|(count, from, to)| {
            let stack_size = stacks[from].len();
            let mut crates = stacks[from].split_off(stack_size - count);
            crates.reverse();
            let mut crates = crates.to_vec();
            stacks[to].append(crates.as_mut());
        });

    stacks
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .fold("".to_string(), |message, ch| {
            format!("{}{}", message, ch).to_owned()
        })
}

fn process_input_second(raw: &str) -> String {
    let mut stacks = Vec::new();

    let (first_half, second_half) = raw.split_once("\n\n").unwrap();

    let first_half = first_half.split("\n").collect::<Vec<&str>>();
    for _ in 0..((first_half[0].len() + 1) / 4) {
        stacks.push(Vec::new());
    }
    for line in first_half {
        let mut stack = 0;
        let chars = line.chars();

        for char in chars.skip(1).step_by(4) {
            if char == ' ' {
                stack += 1;
                continue;
            }
            stacks[stack].push(char);
            stack += 1;
        }
    }

    stacks.iter_mut().for_each(|stack| {
        stack.reverse();
    });

    second_half
        .split("\n")
        .map(|line| {
            let command_raw = line.split(" ").collect::<Vec<&str>>();
            (
                command_raw[1].parse::<usize>().unwrap(),
                command_raw[3].parse::<usize>().unwrap() - 1,
                command_raw[5].parse::<usize>().unwrap() - 1,
            )
        })
        .for_each(|(count, from, to)| {
            let stack_size = stacks[from].len();
            let crates = stacks[from].split_off(stack_size - count);
            let mut crates = crates.to_vec();
            stacks[to].append(crates.as_mut());
        });

    stacks
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .fold("".to_string(), |message, ch| {
            format!("{}{}", message, ch).to_owned()
        })
}
