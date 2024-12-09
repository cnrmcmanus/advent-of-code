mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

fn main() {
    let day = std::env::args().nth(1).unwrap_or("0".to_string());
    match day.as_str() {
        "01" => day01::main(),
        "02" => day02::main(),
        "03" => day03::main(),
        "04" => day04::main(),
        "05" => day05::main(),
        "06" => day06::main(),
        "07" => day07::main(),
        "08" => day08::main(),
        "09" => day09::main(),
        _ => {}
    }
}
