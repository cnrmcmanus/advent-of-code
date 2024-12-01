use std::cmp::Ordering;
use utils::*;

fn find_pair_sum(numbers: &[u32], goal: u32) -> Option<u32> {
    for i in 0..numbers.len() {
        let x = numbers[i];
        for &y in &numbers[i + 1..] {
            match (x + y).cmp(&goal) {
                Ordering::Equal => return Some(x * y),
                Ordering::Greater => break,
                Ordering::Less => {}
            }
        }
    }
    None
}

fn find_triple_sum(numbers: &[u32], goal: u32) -> Option<u32> {
    for i in 0..numbers.len() {
        let x = numbers[i];
        if let Some(partial) = find_pair_sum(&numbers[i + 1..], goal - x) {
            return Some(x * partial);
        }
    }
    None
}

pub fn main() {
    let mut numbers: Vec<u32> = stdin_lines().filter_map(|n| n.parse().ok()).collect();
    numbers.sort();
    println!("{}", find_pair_sum(&numbers, 2020).unwrap_or_default());
    println!("{}", find_triple_sum(&numbers, 2020).unwrap_or_default());
}
