#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

use utils::*;

fn check(grid: &Matrix<char>, index: Point) -> bool {
    (DIRECTIONS.iter())
        .map(|&dir| index + dir)
        .map(|i| u32::from(grid.in_bounds(i) && grid[i] == '@'))
        .sum::<u32>()
        < 4
}

fn removeable(grid: &Matrix<char>, rolls: &[Point]) -> Vec<Point> {
    rolls
        .iter()
        .filter(|&&roll| check(grid, roll))
        .cloned()
        .collect_vec()
}

pub fn main() {
    let mut grid = Matrix::from(
        stdin_lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec(),
    );
    let mut rolls = grid.positions('@').collect_vec();
    let mut pass = removeable(&grid, &rolls);
    let mut removed = pass.len();
    println!("{}", removed);

    while pass.len() > 0 {
        pass.iter().for_each(|&roll| grid[roll] = '.');
        rolls = grid.positions('@').collect_vec();
        pass = removeable(&grid, &rolls);
        removed += pass.len();
    }
    println!("{}", removed);
}
