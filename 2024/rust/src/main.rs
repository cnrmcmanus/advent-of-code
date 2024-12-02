mod day01;
mod day02;

fn main() {
    let day = std::env::args().nth(1).unwrap_or("0".to_string());
    match day.as_str() {
        "01" => day01::main(),
        "02" => day02::main(),
        _ => {}
    }
}
