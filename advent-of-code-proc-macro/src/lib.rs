extern crate proc_macro;
use proc_macro::TokenStream;

/// Returns the necessary `run` function
///
/// # Arguments
///
/// * `problem_day` - A representation of the Problem Year and Day e.g. 2021_17
///
/// # Examples
///
/// ```
/// aoc_pm::make_run!(2023_01)
/// ```
#[proc_macro]
pub fn make_run(problem_day: TokenStream) -> TokenStream {
    let f = problem_day.to_string();
    match f.as_bytes() {
        [y1, y2, y3, y4, b'_', d1, d2]
            if y1 <= &b'9'
                && y1 >= &b'0'
                && y2 <= &b'9'
                && y2 >= &b'0'
                && y3 <= &b'9'
                && y3 >= &b'0'
                && y4 <= &b'9'
                && y4 >= &b'0'
                && d1 <= &b'3'
                && d1 >= &b'0'
                && d2 <= &b'9'
                && d2 >= &b'0' =>
        {
            ()
        }
        _ => panic!("input {f} must be of format YYYY_DD"),
    };
    let year = f.split('_').next().expect("input should be YYYY_DD format");
    let day = f.split('_').last().expect("input should be YYYY_DD format");
    eprintln!("inside proc macro make_answer, f is {f}, day is {day}");
    r##"
use crate::year_YEAR_PLACEHOLDER::Data;

pub fn run() {
    println!("--------------------------");
    println!("Advent of Code YEAR_PLACEHOLDER Day DAY_PLACEHOLDER");
    println!("--------------------------\n");

    let sample_01_data_cow = Data::get("day_DAY_PLACEHOLDER_sample_1").expect("src/year_YEAR_PLACEHOLDER/day_DAY_PLACEHOLDER_sample_1 does not exist").data;
    let sample_01_data = std::str::from_utf8(&sample_01_data_cow).expect("sample_01_data_cow must be a string");

    let sample_02_data_cow = Data::get("day_DAY_PLACEHOLDER_sample_2").expect("src/year_YEAR_PLACEHOLDER/day_DAY_PLACEHOLDER_sample_2 does not exist").data;
    let sample_02_data = std::str::from_utf8(&sample_02_data_cow).expect("sample_02_data_cow must be a string");

    let part_01_data_cow = Data::get("day_DAY_PLACEHOLDER_part_1").expect("src/year_YEAR_PLACEHOLDER/day_DAY_PLACEHOLDER_part_1 does not exist").data;
    let part_01_data = std::str::from_utf8(&part_01_data_cow).expect("part_01_data_cow must be a string");

    let part_02_data_cow = Data::get("day_DAY_PLACEHOLDER_part_1").expect("src/year_YEAR_PLACEHOLDER/day_DAY_PLACEHOLDER_part_2 does not exist").data;
    let part_02_data = std::str::from_utf8(&part_02_data_cow).expect("part_02_data_cow must be a string");

    let start = std::time::SystemTime::now();
    println!(
        "sample:  first half: {}, second half: {}",
        part_1(sample_01_data),
        part_2(sample_02_data)
    );
    println!(
        "actual: first half: {}, second half: {}\n",
        part_1(part_01_data),
        part_2(part_02_data)
    );
    println!("Day DAY_PLACEHOLDER completed in: {:?}\n", start.elapsed().unwrap());
}
"##.replace("YEAR_PLACEHOLDER", year).replace("DAY_PLACEHOLDER", day).parse().unwrap()
}
