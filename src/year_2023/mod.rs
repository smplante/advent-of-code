use rust_embed::RustEmbed;

pub mod day_01;

#[derive(RustEmbed)]
#[folder = "src/year_2023/data"]
pub struct Data;

pub fn run() {
    let start = std::time::SystemTime::now();
    day_01::run();
    println!("All days completed in: {:?}\n\n", start.elapsed().unwrap());
}
