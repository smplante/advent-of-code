aoc_pm::run!(2023, 01, 142, 281);

const NUMS: [(&[u8], u8); 9] = [
    (b"one", b'1'),
    (b"two", b'2'),
    (b"three", b'3'),
    (b"four", b'4'),
    (b"five", b'5'),
    (b"six", b'6'),
    (b"seven", b'7'),
    (b"eight", b'8'),
    (b"nine", b'9'),
];

pub fn part_1(input: &str) -> u32 {
    input.lines().map(parse_line_1).sum()
}

fn parse_line_1(line: &str) -> u32 {
    let line_bytes = line.as_bytes();
    let mut n = 0;

    let mut idx = 0;
    while idx < line_bytes.len() {
        if line_bytes[idx].is_ascii_digit() {
            n += 10 * (line_bytes[idx] - b'0');
            break;
        }

        idx += 1;
    }

    n += line_bytes.iter().rfind(|b| b.is_ascii_digit()).unwrap() - b'0';

    n as u32
}

pub fn part_2(input: &str) -> u32 {
    input.lines().map(parse_line_2).sum()
}

fn parse_line_2(line: &str) -> u32 {
    let line_bytes = line.as_bytes();
    let mut n = 0;

    'outer: for idx in 0..line_bytes.len() {
        if line_bytes[idx].is_ascii_digit() {
            n += 10 * (line_bytes[idx] - b'0');
            break 'outer;
        }

        let rest = &line_bytes[idx..];
        for (word, value) in NUMS {
            if rest.starts_with(word) {
                n += 10 * (value - b'0');
                break 'outer;
            }
        }
    }

    'outer: for idx in (0..line_bytes.len()).rev() {
        if line_bytes[idx].is_ascii_digit() {
            n += line_bytes[idx] - b'0';
            break 'outer;
        }

        let rest = &line_bytes[..=idx];
        for (word, value) in NUMS {
            if rest.ends_with(word) {
                n += value - b'0';
                break 'outer;
            }
        }
    }

    n as u32
}
