#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_mut,
    unused_assignments
)]

use crate::util::*;

fn transpose<T: Copy>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut out = vec![];

    for i in 0..matrix[0].len() {
        let mut col = vec![];
        for row in matrix {
            col.push(row[i]);
        }
        out.push(col);
    }

    out
}

fn find_mirror(map: &[Vec<char>], multiplier: usize, ignore_result: usize) -> Option<usize> {
    for i in 0..map.len() - 1 {
        if map[i] == map[i + 1] {
            if i == 0 || i + 1 == map.len() - 1 {
                let result = (i + 1) * multiplier;
                if result != ignore_result {
                    return Some(result);
                }
            }
            let max = std::cmp::min(i, map.len() - 1 - (i + 1));
            let mirrored = (1..=max).all(|j| map[i - j] == map[(i + 1) + j]);
            if mirrored {
                let result = (i + 1) * multiplier;
                if result != ignore_result {
                    return Some(result);
                }
            }
        }
    }

    None
}

fn mirror(map: &[Vec<char>], ignore_result: usize) -> usize {
    match find_mirror(map, 100, ignore_result) {
        Some(i) => i,
        None => find_mirror(&transpose(map), 1, ignore_result).unwrap_or(0),
    }
}

fn solve(map: Vec<Vec<char>>) -> (usize, usize) {
    let result = mirror(&map, 0);
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let mut copy = map.clone();
            copy[i][j] = if map[i][j] == '.' { '#' } else { '.' };
            let new_result = mirror(&copy, result);
            if new_result != 0 && new_result != result {
                return (result, new_result);
            }
        }
    }
    (0, 0)
}

pub fn main() {
    let (total_1, total_2) = line_chunks(stdin_lines())
        .into_iter()
        .map(|lines| {
            let map = lines
                .into_iter()
                .map(|line| line.chars().collect())
                .collect();
            solve(map)
        })
        .sum_tuples();

    println!("{}\n{}", total_1, total_2);
}
