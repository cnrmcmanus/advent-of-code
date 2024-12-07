mod matrix;

use regex::Regex;
use std::{
    io::{BufRead, Read},
    str::FromStr,
};

pub use itertools::Itertools;
pub use matrix::*;

pub fn stdin<T, F: Fn(String) -> T>(f: F) -> Vec<T> {
    stdin_lines().map(&f).collect()
}

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

pub fn regex(re: &str) -> Regex {
    Regex::new(re).expect("failed to parse regex")
}

pub fn wrap(index: isize, max: usize) -> usize {
    let max = max as isize;
    (((index % max) + max) % max) as usize
}

pub fn magnitude(n: u64) -> u32 {
    if n == 0 {
        1
    } else {
        n.ilog10() + 1
    }
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
