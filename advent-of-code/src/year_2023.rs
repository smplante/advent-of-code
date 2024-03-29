use rust_embed::RustEmbed;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_20;
pub mod day_21;

#[derive(RustEmbed)]
#[folder = "src/year_2023/data"]
pub struct Data;

pub fn run() {
    let start = std::time::SystemTime::now();
    day_01::run();
    day_02::run();
    day_03::run();
    day_04::run();
    day_05::run();
    day_06::run();
    day_07::run();
    day_08::run();
    day_09::run();
    day_10::run();
    day_11::run();
    day_12::run();
    day_13::run();
    day_14::run();
    day_15::run();
    day_16::run();
    // day_17::run();
    day_18::run();
    day_19::run();
    day_20::run();
    day_21::run();
    println!("All days completed in: {:?}\n\n", start.elapsed().unwrap());
}
