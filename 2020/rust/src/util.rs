use std::io::BufRead;

pub fn stdin_lines() -> impl Iterator<Item = String> {
    std::io::stdin().lock().lines().map_while(Result::ok)
}
