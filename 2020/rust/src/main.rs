mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod util;

fn main() {
    let day = std::env::args().nth(1).unwrap_or_default();
    match day.as_str() {
        "01" => day01::main(),
        "02" => day02::main(),
        "03" => day03::main(),
        "04" => day04::main(),
        "05" => day05::main(),
        "06" => day06::main(),
        _ => {}
    }
}
