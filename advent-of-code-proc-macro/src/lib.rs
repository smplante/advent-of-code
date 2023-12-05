extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};

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

    r##"
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

"##.replace("{YEAR}", &year)
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

    let group = inputs.next().unwrap();
    let year = inputs.next().unwrap();
    let day = inputs.next().unwrap();
    r##"
let d = aoc_lib::year_{YEAR}::Data::get("day_{DAY}_part_1")
    .expect("file to exist")
    .data;
let data = std::str::from_utf8(&d).expect("to be a string");
{GROUP}.bench_function("Day {DAY} Part 1", |b| {
    b.iter(|| aoc_lib::year_{YEAR}::day_{DAY}::part_1(criterion::black_box(data)))
});
{GROUP}.bench_function("Day {DAY} Part 2", |b| {
    b.iter(|| aoc_lib::year_{YEAR}::day_{DAY}::part_2(criterion::black_box(data)))
});
{GROUP}.bench_function("Day {DAY}", |b| b.iter(|| aoc_lib::year_{YEAR}::day_{DAY}::run_no_print()));
"##
    .replace("{GROUP}", &group)
    .replace("{YEAR}", &year)
    .replace("{DAY}", &day)
    .parse()
    .unwrap()
}
