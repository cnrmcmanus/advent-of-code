use itertools::Itertools;
use utils::*;

fn xmas(matrix: &[Vec<char>], (i_step, j_step): (isize, isize)) -> u32 {
    (0..matrix.len() as isize)
        .cartesian_product(0..matrix[0].len() as isize)
        .map(|(i, j)| {
            u32::from(
                (['X', 'M', 'A', 'S'].into_iter().enumerate()).all(|(offset, c)| {
                    let i = i.wrapping_add(i_step * offset as isize) as usize;
                    let j = j.wrapping_add(j_step * offset as isize) as usize;
                    i < matrix.len() && j < matrix[0].len() && matrix[i][j] == c
                }),
            )
        })
        .sum()
}

fn crossmas(matrix: &[Vec<char>]) -> u32 {
    (1..matrix.len() - 1)
        .cartesian_product(1..matrix[0].len() - 1)
        .map(|(i, j)| {
            let top_pair = (matrix[i - 1][j - 1], matrix[i - 1][j + 1]);
            let bottom_pair = (matrix[i + 1][j - 1], matrix[i + 1][j + 1]);
            let left_pair = (matrix[i - 1][j - 1], matrix[i + 1][j - 1]);
            let right_pair = (matrix[i - 1][j + 1], matrix[i + 1][j + 1]);
            let check = |pair_1, pair_2| pair_1 == ('S', 'S') && pair_2 == ('M', 'M');
            u32::from(
                matrix[i][j] == 'A'
                    && (check(bottom_pair, top_pair)
                        || check(top_pair, bottom_pair)
                        || check(left_pair, right_pair)
                        || check(right_pair, left_pair)),
            )
        })
        .sum()
}

pub fn main() {
    let grid: Vec<Vec<char>> = stdin_lines().map(|line| line.chars().collect()).collect();

    let all_xmas: u32 = (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|&step| step != (0, 0))
        .map(|step| xmas(&grid, step))
        .sum();
    println!("{}", all_xmas);

    println!("{}", crossmas(&grid));
}
