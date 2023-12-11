use std::cmp::max;

pub fn part_1(input: &str) -> Option<u32> {
    input.lines().map(parse_line_1).sum()
}

fn parse_line_1(line: &str) -> Option<u32> {
    let mut ls = line.split([':', ';', ',']);
    let game = ls.next()?;
    let value = game
        .split_ascii_whitespace()
        .last()?
        .trim()
        .parse::<u32>()
        .ok()?;

    for pull in ls {
        let mut ps = pull.split_ascii_whitespace();
        let num = ps.next()?;
        match ps.last()? {
            "red" => {
                if num.parse::<u32>().ok()? > 12 {
                    return Some(0);
                }
            }
            "green" => {
                if num.parse::<u32>().ok()? > 13 {
                    return Some(0);
                }
            }
            "blue" => {
                if num.parse::<u32>().ok()? > 14 {
                    return Some(0);
                }
            }
            _ => todo!(),
        }
    }

    Some(value)
}

pub fn part_2(input: &str) -> Option<u32> {
    input.lines().map(parse_line_2).sum()
}

fn parse_line_2(line: &str) -> Option<u32> {
    let mut ls = line.split([':', ';', ',']);
    _ = ls.next()?;

    let (mut red, mut green, mut blue) = (0, 0, 0);

    for pull in ls {
        let mut ps = pull.split_ascii_whitespace();
        let num = ps.next()?;
        match ps.last()? {
            "red" => red = max(red, num.parse::<u32>().ok()?),
            "green" => green = max(green, num.parse::<u32>().ok()?),
            "blue" => blue = max(blue, num.parse::<u32>().ok()?),
            _ => unreachable!(),
        }
    }

    Some(red * green * blue)
}
