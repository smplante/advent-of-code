use rust_embed::RustEmbed;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;

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
    println!("All days completed in: {:?}\n\n", start.elapsed().unwrap());
}
