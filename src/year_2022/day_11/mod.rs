mod inputs;

pub fn run() {
    let start = std::time::SystemTime::now();
    println!("--------------------------");
    println!("Advent of Code 2022 Day 11");
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
    println!("Day 11 completed in: {:?}\n", start.elapsed().unwrap());
}

// #[derive(Debug)]
struct Monkey<O>
where
    O: Fn(u128) -> u128,
{
    inspections: u128,
    items: Vec<u128>,
    operation: O,
    divisor: u128,
    on_true: usize,
    on_false: usize,
}

impl<O> Monkey<O>
where
    O: Fn(u128) -> u128,
{
    fn new(
        items: Vec<u128>,
        operation: O,
        divisor: u128,
        on_true: usize,
        on_false: usize,
    ) -> Monkey<O> {
        Monkey {
            inspections: 0,
            items,
            operation,
            divisor,
            on_true,
            on_false,
        }
    }
}

fn process_input_first(input: &str) -> u128 {
    let mut monkeys = Vec::new();

    let mut items: Vec<u128> = Vec::new();
    let mut operation: Box<dyn Fn(u128) -> u128> = Box::new(|old| old + u128::from(1u128));
    let mut divisor: u128 = 0;
    let mut on_true = 0;
    let mut on_false = 0;

    for line in input.split("\n") {
        if line == "" {
            monkeys.push(Monkey::new(items, operation, divisor, on_true, on_false));
            items = Vec::new();
            operation = Box::new(|old| old + u128::from(1u128));
            continue;
        }
        match line.trim().split_once(":").unwrap() {
            (a, _) if a.starts_with("Monkey") => (),
            (a, b) if a.starts_with("Starting") => {
                items = b
                    .trim()
                    .split(",")
                    .map(str::trim)
                    .map(str::parse::<u128>)
                    .map(Result::unwrap)
                    .collect::<Vec<u128>>();
            }
            (a, b) if a.starts_with("Operation") => {
                let mut equation_parts = b
                    .split_once("=")
                    .unwrap()
                    .1
                    .trim()
                    .splitn(3, " ")
                    .into_iter();
                match (
                    equation_parts.next().unwrap(),
                    equation_parts.next().unwrap(),
                    equation_parts.next().unwrap(),
                ) {
                    (a, "+", b) => {
                        operation = Box::new(|old| {
                            a.parse::<u128>().unwrap_or(old.clone())
                                + b.parse::<u128>().unwrap_or(old)
                        });
                    }
                    (a, "*", b) => {
                        operation = Box::new(|old| {
                            a.parse::<u128>().unwrap_or(old.clone())
                                * b.parse::<u128>().unwrap_or(old)
                        });
                    }
                    _ => (),
                }
            }
            (a, b) if a.starts_with("Test") => {
                divisor = b
                    .trim()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u128>()
                    .unwrap();
            }
            (a, b) if a.starts_with("If true") => {
                on_true = b
                    .trim()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
            }
            (a, b) if a.starts_with("If false") => {
                on_false = b
                    .trim()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
            }
            _ => (),
        }
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let on_true = monkeys[i].on_true.to_owned();
            let on_false = monkeys[i].on_false.to_owned();
            while monkeys.get(i).unwrap().items.len() > 0 {
                let item = monkeys.get_mut(i).unwrap().items.remove(0);
                let mut worry_level = (monkeys[i].operation)(item);
                worry_level = worry_level / 3;
                if worry_level.clone() % monkeys[i].divisor == u128::from(0u128) {
                    monkeys[on_true].items.push(worry_level);
                } else {
                    monkeys[on_false].items.push(worry_level);
                }

                monkeys.get_mut(i).unwrap().inspections += 1;
            }
        }
    }

    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<u128>>();
    inspections.sort();

    inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
}

fn process_input_second(input: &str) -> u128 {
    let mut monkeys = Vec::new();

    let mut items: Vec<u128> = Vec::new();
    let mut operation: Box<dyn Fn(u128) -> u128> = Box::new(|old| old + u128::from(1u128));
    let mut divisor: u128 = 0;
    let mut on_true = 0;
    let mut on_false = 0;

    for line in input.split("\n") {
        if line == "" {
            monkeys.push(Monkey::new(items, operation, divisor, on_true, on_false));
            items = Vec::new();
            operation = Box::new(|old| old + u128::from(1u128));
            continue;
        }
        match line.trim().split_once(":").unwrap() {
            (a, _) if a.starts_with("Monkey") => (),
            (a, b) if a.starts_with("Starting") => {
                items = b
                    .trim()
                    .split(",")
                    .map(str::trim)
                    .map(str::parse::<u128>)
                    .map(Result::unwrap)
                    .collect::<Vec<u128>>();
            }
            (a, b) if a.starts_with("Operation") => {
                let mut equation_parts = b
                    .split_once("=")
                    .unwrap()
                    .1
                    .trim()
                    .splitn(3, " ")
                    .into_iter();
                match (
                    equation_parts.next().unwrap(),
                    equation_parts.next().unwrap(),
                    equation_parts.next().unwrap(),
                ) {
                    (a, "+", b) => {
                        operation = Box::new(|old| {
                            a.parse::<u128>().unwrap_or(old.clone())
                                + b.parse::<u128>().unwrap_or(old)
                        });
                    }
                    (a, "*", b) => {
                        operation = Box::new(|old| {
                            a.parse::<u128>().unwrap_or(old.clone())
                                * b.parse::<u128>().unwrap_or(old)
                        });
                    }
                    _ => (),
                }
            }
            (a, b) if a.starts_with("Test") => {
                divisor = b
                    .trim()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u128>()
                    .unwrap();
            }
            (a, b) if a.starts_with("If true") => {
                on_true = b
                    .trim()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
            }
            (a, b) if a.starts_with("If false") => {
                on_false = b
                    .trim()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
            }
            _ => (),
        }
    }

    let all_divisors = monkeys.iter().map(|m| m.divisor).product::<u128>();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let on_true = monkeys[i].on_true.to_owned();
            let on_false = monkeys[i].on_false.to_owned();
            while monkeys.get(i).unwrap().items.len() > 0 {
                let item = monkeys.get_mut(i).unwrap().items.remove(0);
                let mut worry_level = (monkeys[i].operation)(item);
                worry_level = worry_level % u128::from(all_divisors);
                if worry_level.clone() % monkeys[i].divisor == u128::from(0u128) {
                    monkeys[on_true].items.push(worry_level);
                } else {
                    monkeys[on_false].items.push(worry_level);
                }

                monkeys.get_mut(i).unwrap().inspections += 1;
            }
        }
    }

    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<u128>>();
    inspections.sort();

    inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
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
