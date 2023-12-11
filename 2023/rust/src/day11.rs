use crate::util::*;

fn distance(a: usize, b: usize, gaps: &[usize], gap_size: usize) -> usize {
    let small = std::cmp::min(a, b);
    let big = std::cmp::max(a, b);
    let gap_count = gaps.iter().filter(|&&gap| small < gap && gap < big).count();
    (big - small) + (gap_count * gap_size) - gap_count
}

pub fn main() {
    let map: Vec<Vec<char>> = stdin_lines().map(|line| line.chars().collect()).collect();
    let empty_rows: Vec<_> = map
        .iter()
        .enumerate()
        .filter_map(|(i, row)| row.iter().all(|&c| c == '.').then_some(i))
        .collect();
    let mut empty_columns = vec![];
    for j in 0..map[0].len() {
        if map.iter().all(|row| row[j] == '.') {
            empty_columns.push(j);
        }
    }
    let mut galaxies = vec![];
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }

    let (mut total_1, mut total_2) = (0, 0);
    for (i, galaxy_1) in galaxies.clone().into_iter().enumerate() {
        for &galaxy_2 in &galaxies[i + 1..] {
            total_1 += distance(galaxy_1.0, galaxy_2.0, &empty_rows, 2)
                + distance(galaxy_1.1, galaxy_2.1, &empty_columns, 2);

            total_2 += distance(galaxy_1.0, galaxy_2.0, &empty_rows, 1000000)
                + distance(galaxy_1.1, galaxy_2.1, &empty_columns, 1000000);
        }
    }

    println!("{}\n{}", total_1, total_2);
}
