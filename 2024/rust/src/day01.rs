use utils::*;

pub fn main() {
    let (mut col1, mut col2): (Vec<_>, Vec<_>) = stdin_lines()
        .filter_map(|line| {
            let mut cols = line.split_and_parse::<i32>("   ");
            cols.next().zip(cols.next())
        })
        .unzip();
    col1.sort();
    col2.sort();

    let distance = (col1.iter().zip(col2.iter()))
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();
    println!("{}", distance);

    let similarity = (col1.iter())
        .map(|&a| a * col2.iter().filter(|&&b| a == b).count() as i32)
        .sum::<i32>();
    println!("{}", similarity);
}
