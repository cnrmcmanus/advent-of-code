use std::collections::HashSet;
use utils::*;

pub fn main() {
    let groups: Vec<Vec<HashSet<char>>> = stdin_lines_by(r"(\r\n\r\n|\n\n)")
        .into_iter()
        .map(|group| group.lines().map(|s| s.chars().collect()).collect())
        .collect();

    let unique_total: usize = groups
        .iter()
        .map(|group| {
            group
                .iter()
                .fold(group[0].clone(), |mut acc, answers| {
                    acc.extend(answers.clone());
                    acc
                })
                .len()
        })
        .sum();
    println!("{}", unique_total);

    let common_total: usize = groups
        .iter()
        .map(|group| {
            group
                .iter()
                .skip(1)
                .fold(group[0].clone(), |intersection, answers| {
                    intersection.intersection(answers).cloned().collect()
                })
                .len()
        })
        .sum();
    println!("{}", common_total);
}
