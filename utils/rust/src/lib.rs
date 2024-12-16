mod matrix;
mod point;

use regex::Regex;
use std::{
    io::{BufRead, Read},
    str::FromStr,
};

pub use itertools::{Either, Itertools};
pub use std::collections::{HashMap, HashSet};
pub use std::cmp::{Ordering, Reverse};

pub use derive_new::new;

pub use matrix::*;
pub use point::*;

pub fn stdin<T, F: Fn(String) -> T>(f: F) -> Vec<T> {
    stdin_lines().map(&f).collect()
}

pub fn stdin_lines() -> impl Iterator<Item = String> {
    std::io::stdin().lock().lines().map_while(Result::ok)
}

pub fn stdin_all() -> String {
    let mut contents = String::new();
    std::io::stdin().lock().read_to_string(&mut contents).ok();
    while let Some(c) = contents.chars().last() {
        if c.is_control() {
            contents.pop();
        } else {
            break;
        }
    }
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

pub trait IteratorExtensions: Iterator {
    fn map_sum<U, F>(self, mut f: F) -> U
    where
        Self: Sized,
        U: Default + std::ops::Add<U, Output = U>,
        F: FnMut(Self::Item) -> U,
    {
        let initial = U::default();
        self.fold(initial, |sum, x| sum + f(x))
    }
}

impl<T: Iterator> IteratorExtensions for T {}

pub trait VecExtensions<T> {
    fn sorted_insert_by<F>(&mut self, a: T, cmp: F) where F: Fn(&T, &T) -> Ordering;

    fn sorted_insert_by_key<U: Ord, F>(&mut self, a: T, key: F) where F: Fn(&T) -> U {
        self.sorted_insert_by(a, |a, b| key(a).cmp(&key(b)));
    }
}

impl<T> VecExtensions<T> for Vec<T> {
    fn sorted_insert_by<F>(&mut self, a: T, cmp: F) where F: Fn(&T, &T) -> Ordering {
        let i = self.binary_search_by(|b| cmp(&a, b)).unwrap_or_else(|i| i);
        self.insert(i, a);
    }
}
