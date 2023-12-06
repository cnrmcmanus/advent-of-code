use std::io::BufRead;
use std::str::FromStr;

pub fn stdin_lines() -> impl Iterator<Item = String> {
    std::io::stdin().lock().lines().map_while(Result::ok)
}

pub fn str_to_vec<T: FromStr>(string: &str) -> Vec<T> {
    string.split(' ').filter_map(|n| n.parse().ok()).collect()
}
