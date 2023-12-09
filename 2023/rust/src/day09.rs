use crate::util::*;

fn differences(numbers: &[i64]) -> Vec<i64> {
    numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect()
}

pub fn main() {
    let (total_2, total_1): (i64, i64) = stdin_lines()
        .map(|line| {
            let top_row: Vec<i64> = str_to_vec(&line);
            let rows: Vec<Vec<i64>> = std::iter::successors(Some(top_row), |prev_row| {
                let row = differences(prev_row);
                (!row.iter().all(|&n| n == 0)).then_some(row)
            })
            .collect();

            rows.into_iter().rev().fold((0, 0), |(prev, next), row| {
                (row.first().unwrap() - prev, row.last().unwrap() + next)
            })
        })
        .sum_tuples();

    println!("{}\n{}", total_1, total_2);
}
