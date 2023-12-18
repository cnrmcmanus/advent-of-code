mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day11;
mod day12;
mod day13;
mod day17;
mod day18;
mod util;

fn main() {
    let day = std::env::args().nth(1).unwrap_or("0".to_string());
    match day.as_str() {
        "04" => day04::main(),
        "05" => day05::main(),
        "06" => day06::main(),
        "07" => day07::main(),
        "08" => day08::main(),
        "09" => day09::main(),
        "11" => day11::main(),
        "12" => day12::main(),
        "13" => day13::main(),
        "17" => day17::main(),
        "18" => day18::main(),
        _ => {}
    }
}
