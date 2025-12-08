mod day01;
mod day03;
mod day08;

fn main() {
    let day = std::env::args().nth(1).unwrap_or("0".to_string());
    match day.as_str() {
        "01" => day01::main(),
        "03" => day03::main(),
        "08" => day08::main(),
        _ => {}
    }
}
