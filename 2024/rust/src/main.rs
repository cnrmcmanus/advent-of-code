mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day16;
mod day18;
mod day19;

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
        "10" => day10::main(),
        "11" => day11::main(),
        "12" => day12::main(),
        "16" => day16::main(),
        "18" => day18::main(),
        "19" => day19::main(),
        _ => {}
    }
}
