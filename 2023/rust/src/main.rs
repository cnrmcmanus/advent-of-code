mod day05;
mod util;

fn main() {
    let day = std::env::args().nth(1).unwrap_or("0".to_string());
    match day.as_str() {
        "05" => day05::main(),
        _ => {}
    }
}
