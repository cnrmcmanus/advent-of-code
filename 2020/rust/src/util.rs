use regex::Regex;
use std::io::{BufRead, Read};

pub fn stdin_lines() -> impl Iterator<Item = String> {
    std::io::stdin().lock().lines().map_while(Result::ok)
}

pub fn stdin_all() -> String {
    let mut contents = String::new();
    std::io::stdin().lock().read_to_string(&mut contents).ok();
    contents
}

pub fn stdin_lines_by(pattern: &str) -> Vec<String> {
    let re = Regex::new(pattern).unwrap();
    re.split(&stdin_all()).map(ToOwned::to_owned).collect()
}
