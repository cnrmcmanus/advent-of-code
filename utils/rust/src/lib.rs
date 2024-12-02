use regex::Regex;
use std::{
    io::{BufRead, Read},
    str::FromStr,
};

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

pub trait StrExtensions {
    fn parse_ok<F: FromStr>(&self) -> Option<F>;

    fn split_and_parse<T: FromStr>(&self, delim: &str) -> impl Iterator<Item = T>;
}

impl StrExtensions for str {
    fn parse_ok<F: FromStr>(&self) -> Option<F> {
        self.parse::<F>().ok()
    }

    fn split_and_parse<T: FromStr>(&self, delim: &str) -> impl Iterator<Item = T> {
        self.split(delim).filter_map(str::parse_ok)
    }
}
