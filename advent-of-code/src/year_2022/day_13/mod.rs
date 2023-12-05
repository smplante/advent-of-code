use std::{cmp::Ordering, fmt};

mod inputs;

pub fn run() {
    let start = std::time::SystemTime::now();
    println!("--------------------------");
    println!("Advent of Code 2022 Day 13");
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
    println!("Day 13 completed in: {:?}\n", start.elapsed().unwrap());
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Packet {
    Array(Vec<Packet>),
    Number(i8),
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Packet::Number(n) => write!(f, "{}", n),
            Packet::Array(a) => {
                if a.is_empty() {
                    return write!(f, "[]");
                }
                write!(f, "[").unwrap();
                for i in a.iter() {
                    write!(f, "{},", i).unwrap();
                }
                write!(f, "{}]", a[a.len() - 1]).unwrap();
                Ok(())
            }
        }
    }
}

impl Packet {
    fn parse(input: &str) -> Packet {
        let mut packets = vec![Vec::new()];
        let mut layer = 0;

        let mut it = input.chars().peekable();

        while it.peek().is_some() {
            let c = it.next().unwrap();
            match c {
                '[' => {
                    packets.push(Vec::new());
                    layer += 1;
                }
                ']' => {
                    let packet = packets.pop().unwrap();
                    layer -= 1;
                    packets[layer].push(Packet::new_array(packet))
                }
                ',' => (),
                '\n' => (),
                n => {
                    if it.peek().is_none()
                        || it.peek().unwrap() == &','
                        || it.peek().unwrap() == &'['
                        || it.peek().unwrap() == &']'
                    {
                        packets[layer]
                            .push(Packet::new_number(&(n.to_string().parse::<i8>().unwrap())))
                    } else {
                        let n2 = it.next().unwrap();
                        let mut new_n = n.to_string();
                        new_n.push(n2);
                        packets[layer].push(Packet::new_number(&new_n.parse::<i8>().unwrap()))
                    }
                }
            }
        }

        packets.remove(0).remove(0)
    }

    fn new_array(v: Vec<Packet>) -> Packet {
        Packet::Array(v)
    }
    fn new_number(n: &i8) -> Packet {
        Packet::Number(*n)
    }

    fn cmp(&self, right: &Self) -> Ordering {
        let b: Ordering = match (self, right) {
            (Packet::Number(l), Packet::Number(r)) => l.cmp(r),
            (Packet::Number(l), r) => {
                let l = Packet::new_array(vec![Packet::new_number(l)]);
                l.cmp(r)
            }
            (l, Packet::Number(r)) => {
                let r = Packet::new_array(vec![Packet::new_number(r)]);
                l.cmp(&r)
            }
            (Packet::Array(l), Packet::Array(r)) => {
                let mut i = 0;
                while i < l.len() && i < r.len() {
                    let compare = l[i].cmp(&r[i]);
                    if compare != Ordering::Equal {
                        return compare;
                    }
                    i += 1;
                }

                l.len().cmp(&r.len())
            }
        };

        b
    }
}

fn process_input_first(input: &str) -> usize {
    let b = input
        .split("\n\n")
        .map(|pairs| {
            let (left, right) = pairs.split_once('\n').unwrap();
            let left = Packet::parse(left);
            let right = Packet::parse(right);

            let b = left.cmp(&right);

            b == Ordering::Less
        })
        .collect::<Vec<bool>>();

    b.iter().enumerate().fold(0, |sum, (i, b)| match &b {
        true => sum + i + 1,
        false => sum,
    })
}

fn process_input_second(input: &str) -> usize {
    let mut packets = input
        .split("\n\n")
        .flat_map(|pairs| {
            let (left, right) = pairs.split_once('\n').unwrap();
            let left = Packet::parse(left);
            let right = Packet::parse(right);

            vec![left, right]
        })
        .collect::<Vec<Packet>>();

    packets.push(Packet::parse("[[2]]"));
    packets.push(Packet::parse("[[6]]"));

    packets.sort_by(|left, right| left.cmp(right));

    packets.iter().enumerate().fold(1, |product, (i, b)| {
        if format!("{}", b) == "[[2]]" || format!("{}", b) == "[[6]]" {
            return product * (i + 1);
        }

        product
    })
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
