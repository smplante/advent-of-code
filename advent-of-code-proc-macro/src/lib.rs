extern crate proc_macro;

use itertools::Itertools;
use proc_macro::{TokenStream, TokenTree};
use regex::bytes::{Match, Regex};

/// Returns the necessary `run` function
///
/// # Arguments
///
/// * `year` - The problem year
/// * `day` - The problem year
/// * `answer_1_sample` - The Part 1 sample answer
/// * `answer_2_sample` - The Part 2 sample answer
///
/// # Examples
///
/// ```
/// //aoc_pm::make_run!(2023, 01, 0, 0);
/// ```
#[proc_macro]
pub fn run(ts: TokenStream) -> TokenStream {
    let mut inputs = ts.into_iter().filter_map(|tt| match tt {
        TokenTree::Literal(t) => Some(t.to_string()),
        TokenTree::Ident(t) => Some(t.to_string()),
        _ => None,
    });

    let year = inputs.next().unwrap();
    let day = inputs.next().unwrap();
    let answer_1_sample = inputs.next().unwrap();
    let answer_2_sample = inputs.next().unwrap();

    r#"
use crate::year_{YEAR}::Data;

pub fn run() {
    eprintln!("--------------------------");
    eprintln!("Advent of Code {YEAR} Day {DAY}");
    eprintln!("--------------------------\n");

    let sample_01_data_cow = Data::get("day_{DAY}_sample_1").expect("src/year_{YEAR}/day_{DAY}_sample_1 does not exist").data;
    let sample_01_data = std::str::from_utf8(&sample_01_data_cow).expect("sample_01_data_cow must be a string");

    let sample_02_data_cow = Data::get("day_{DAY}_sample_2").expect("src/year_{YEAR}/day_{DAY}_sample_2 does not exist").data;
    let sample_02_data = std::str::from_utf8(&sample_02_data_cow).expect("sample_02_data_cow must be a string");

    let part_01_data_cow = Data::get("day_{DAY}_part_1").expect("src/year_{YEAR}/day_{DAY}_part_1 does not exist").data;
    let part_01_data = std::str::from_utf8(&part_01_data_cow).expect("part_01_data_cow must be a string");

    let part_02_data_cow = Data::get("day_{DAY}_part_1").expect("src/year_{YEAR}/day_{DAY}_part_2 does not exist").data;
    let part_02_data = std::str::from_utf8(&part_02_data_cow).expect("part_02_data_cow must be a string");

    let start = std::time::SystemTime::now();
    eprintln!(
        "sample:  first half: {}, second half: {}",
        part_1(sample_01_data),
        part_2(sample_02_data)
    );
    eprintln!(
        "actual: first half: {}, second half: {}\n",
        part_1(part_01_data),
        part_2(part_02_data)
    );
    eprintln!("Day {DAY} completed in: {:?}\n", start.elapsed().unwrap());
}

pub fn run_no_print() {
    let sample_01_data_cow = Data::get("day_{DAY}_sample_1").expect("src/year_{YEAR}/day_{DAY}_sample_1 does not exist").data;
    let sample_01_data = std::str::from_utf8(&sample_01_data_cow).expect("sample_01_data_cow must be a string");

    let sample_02_data_cow = Data::get("day_{DAY}_sample_2").expect("src/year_{YEAR}/day_{DAY}_sample_2 does not exist").data;
    let sample_02_data = std::str::from_utf8(&sample_02_data_cow).expect("sample_02_data_cow must be a string");

    let part_01_data_cow = Data::get("day_{DAY}_part_1").expect("src/year_{YEAR}/day_{DAY}_part_1 does not exist").data;
    let part_01_data = std::str::from_utf8(&part_01_data_cow).expect("part_01_data_cow must be a string");

    let part_02_data_cow = Data::get("day_{DAY}_part_1").expect("src/year_{YEAR}/day_{DAY}_part_2 does not exist").data;
    let part_02_data = std::str::from_utf8(&part_02_data_cow).expect("part_02_data_cow must be a string");

    part_1(sample_01_data);
    part_2(sample_02_data);
    part_1(part_01_data);
    part_2(part_02_data);
}

#[cfg(test)]
mod generated_tests {
    use crate::year_{YEAR}::{day_{DAY}, Data};

    #[test]
    fn part_1_sample() {
        let d = Data::get("day_{DAY}_sample_1")
            .expect("src/year_{YEAR}/day_{DAY}_sample_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_{DAY}::part_1(input), {ANSWER_1_SAMPLE});
    }

    #[test]
    fn part_2_sample() {
        let d = Data::get("day_{DAY}_sample_2")
            .expect("src/year_{YEAR}/day_{DAY}_sample_1 does not exist")
            .data;
        let input = std::str::from_utf8(&d).expect("d must be a string");
        assert_eq!(day_{DAY}::part_2(input), {ANSWER_2_SAMPLE});
    }
}

"#.replace("{YEAR}", &year)
        .replace("{DAY}", &day)
        .replace("{ANSWER_1_SAMPLE}", &answer_1_sample)
        .replace("{ANSWER_2_SAMPLE}", &answer_2_sample)
        .parse().unwrap()
}

#[proc_macro]
pub fn benchmark(ts: TokenStream) -> TokenStream {
    let mut inputs = ts.into_iter().filter_map(|tt| match tt {
        TokenTree::Literal(t) => Some(t.to_string()),
        TokenTree::Ident(t) => Some(t.to_string()),
        _ => None,
    });

    let year = inputs.next().unwrap();
    let day = inputs.next().unwrap();

    let part_1_regex = Regex::new(r##"\npub fn (part_1[^\(\ \"]*)"##).unwrap();
    let part_2_regex = Regex::new(r##"\npub fn (part_2[^\(\ \"]*)"##).unwrap();

    let contents = std::fs::read(format!("advent-of-code/src/year_{year}/day_{day}.rs")).unwrap();

    let part_1_funcs = part_1_regex
        .captures_iter(&contents)
        .map(|m| {
            let (_, [func]) = m.extract();
            func
        })
        .sorted()
        .unique()
        .collect::<Vec<&[u8]>>();

    let part_2_funcs = part_2_regex
        .captures_iter(&contents)
        .map(|m| {
            let (_, [func]) = m.extract();
            func
        })
        .sorted()
        .unique()
        .collect::<Vec<&[u8]>>();

    let mut code_gen = Vec::new();
    code_gen.push(
        r#"let mut group_day_{DAY} = c.benchmark_group("{YEAR} Day {DAY}");
    let input_1 = aoc_lib::year_{YEAR}::Data::get("day_{DAY}_part_1")
        .expect("file to exist")
        .data;
    let input_1 = std::str::from_utf8(&input_1).unwrap();
    let input_2 = aoc_lib::year_{YEAR}::Data::get("day_{DAY}_part_2")
        .expect("file to exist")
        .data;
    let input_2 = std::str::from_utf8(&input_2).unwrap();"#
            .to_string(),
    );

    for func in part_1_funcs {
        let func = std::str::from_utf8(func).unwrap();
        let x = r#"group_day_{DAY}.bench_with_input("{func}", &input_1, |b, i| {
    b.iter(|| aoc_lib::year_{YEAR}::day_{DAY}::{func}(i))
});"#
            .replace("{func}", func);
        code_gen.push(x);
    }

    for func in part_2_funcs {
        let func = std::str::from_utf8(func).unwrap();
        let x = r#"group_day_{DAY}.bench_with_input("{func}", &input_2, |b, i| {
    b.iter(|| aoc_lib::year_{YEAR}::day_{DAY}::{func}(i))
});"#
            .replace("{func}", func);
        code_gen.push(x);
    }

    code_gen.push("group_day_{DAY}.finish();".to_string());

    let r = code_gen
        .join("\n")
        .replace("{YEAR}", &year)
        .replace("{DAY}", &day)
        .parse()
        .unwrap();

    r
}
