use std::cmp::Ordering::{self, *};
use utils::*;

pub fn rule_cmp(start: u32, end: u32, rules: &[Vec<u32>]) -> Ordering {
    (rules.iter())
        .find(|rule| rule.contains(&start) && rule.contains(&end))
        .map_or(Equal, |rule| if start == rule[0] { Less } else { Greater })
}

pub fn main() {
    let rules: Vec<Vec<u32>> = stdin_lines()
        .take_while(|line| line.contains('|'))
        .map(|line| line.split_and_parse("|").collect())
        .collect();
    let updates: Vec<Vec<u32>> = stdin_lines()
        .map(|line| line.split_and_parse(",").collect())
        .collect();

    let (good, bad) = updates.into_iter().fold((0, 0), |(good, bad), update| {
        let mut sorted = update.clone();
        sorted.sort_by(|&n, &m| rule_cmp(n, m, &rules));
        let score = sorted[sorted.len() / 2];
        if update == sorted {
            (good + score, bad)
        } else {
            (good, bad + score)
        }
    });
    println!("{}\n{}", good, bad);
}
