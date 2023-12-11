use itertools::Itertools;

pub fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect_vec()
        })
        .map(find_next)
        .sum()
}

pub fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .rev()
                .collect_vec()
        })
        .map(find_next)
        .sum()
}

fn find_next(seq: Vec<i32>) -> i32 {
    let mut seq_steps = vec![seq];
    let mut i = 0;
    let mut diffs = Vec::new();
    while i < seq_steps.len() {
        for j in 0..(seq_steps[i].len() - 1) {
            diffs.push(seq_steps[i][j + 1] - seq_steps[i][j]);
        }
        if diffs.iter().filter(|&&n| n != 0).count() == 0 {
            break;
        }
        seq_steps.push(diffs);
        diffs = Vec::new();
        i += 1;
    }

    for i in (1..(seq_steps.len())).rev() {
        let v = seq_steps[i].pop().unwrap();
        let pv = seq_steps[i - 1].last().unwrap().to_owned();
        seq_steps[i - 1].push(pv + v);
    }

    seq_steps[0].last().unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use crate::year_2023::{day_09::v1::*, Data};

    #[test]
    fn part_1_actual() {
        let d = Data::get("day_09_part_1")
            .expect("src/year_2023/day_09_part_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_1(input), 1_696_140_818);
    }

    #[test]
    fn part_2_actual() {
        let d = Data::get("day_09_part_2")
            .expect("src/year_2023/day_09_part_2 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(part_2(input), 1_152);
    }
}
