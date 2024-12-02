use utils::*;

fn safe(row_front: &[i32], row_back: &[i32]) -> bool {
    let row = || row_front.iter().chain(row_back.iter());
    let zipped = || row().zip(row().skip(1));
    let all_inc = zipped().all(|(a, b)| a < b);
    let all_dec = zipped().all(|(a, b)| a > b);
    let all_close = zipped().all(|(a, b)| (1..=3).contains(&(a - b).abs()));
    all_close && (all_inc || all_dec)
}

fn loose_safe(row: &[i32]) -> bool {
    safe(row, &[]) || (0..row.len()).any(|i| safe(&row[0..i], &row[i + 1..]))
}

pub fn main() {
    let grid: Vec<Vec<i32>> = stdin_lines()
        .map(|line| line.split_and_parse(" ").collect())
        .collect();

    let all_safe = grid.iter().filter(|row| safe(row, &[])).count();
    println!("{}", all_safe);

    let loosley_safe = grid.iter().filter(|row| loose_safe(row)).count();
    println!("{}", loosley_safe);
}
