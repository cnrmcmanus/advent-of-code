mod day01;
mod day02;
mod day03;
mod util;

fn main() {
    let day = std::env::args().nth(1).unwrap_or_default();
    match day.as_str() {
        "01" => day01::main(),
        "02" => day02::main(),
        "03" => day03::main(),
        _ => {}
    }
}
