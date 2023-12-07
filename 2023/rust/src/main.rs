mod day05;
mod day06;
mod day07;
mod util;

fn main() {
    let day = std::env::args().nth(1).unwrap_or("0".to_string());
    match day.as_str() {
        "05" => day05::main(),
        "06" => day06::main(),
        "07" => day07::main(),
        _ => {}
    }
}
