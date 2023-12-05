use std::cmp::max;

aoc_pm::run!(2023, 02, 8, 2286);

pub fn part_1(input: &str) -> u32 {
    input.lines().map(parse_line_1).sum()
}

fn parse_line_1(line: &str) -> u32 {
    let mut ls = line.split([':', ';', ',']);
    let game = ls.next().unwrap();
    let value = game
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .trim()
        .parse::<u32>()
        .unwrap();

    for pull in ls {
        let mut ps = pull.split_ascii_whitespace();
        let num = ps.next().unwrap();
        match ps.last().unwrap() {
            "red" => {
                if num.parse::<u32>().unwrap() > 12 {
                    return 0;
                }
            }
            "green" => {
                if num.parse::<u32>().unwrap() > 13 {
                    return 0;
                }
            }
            "blue" => {
                if num.parse::<u32>().unwrap() > 14 {
                    return 0;
                }
            }
            _ => todo!(),
        }
    }

    value
}

pub fn part_2(input: &str) -> u32 {
    input.lines().map(parse_line_2).sum()
}

fn parse_line_2(line: &str) -> u32 {
    let mut ls = line.split([':', ';', ',']);
    _ = ls.next().unwrap();

    let (mut red, mut green, mut blue) = (0, 0, 0);

    for pull in ls {
        let mut ps = pull.split_ascii_whitespace();
        let num = ps.next().unwrap();
        match ps.last().unwrap() {
            "red" => red = max(red, num.parse::<u32>().unwrap()),
            "green" => green = max(green, num.parse::<u32>().unwrap()),
            "blue" => blue = max(blue, num.parse::<u32>().unwrap()),
            _ => todo!(),
        }
    }

    red * green * blue
}
