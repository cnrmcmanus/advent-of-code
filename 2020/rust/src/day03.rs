use utils::*;

fn descend(grid: &[Vec<char>], right: usize, down: usize) -> usize {
    let width = grid[0].len();
    (0..grid.len())
        .step_by(down)
        .enumerate()
        .filter(|&(step, i)| {
            let j = (step * right) % width;
            grid[i][j] == '#'
        })
        .count()
}

pub fn main() {
    let grid: Vec<Vec<char>> = stdin_lines().map(|line| line.chars().collect()).collect();
    println!("{}", descend(&grid, 3, 1));
    println!(
        "{}",
        descend(&grid, 1, 1)
            * descend(&grid, 3, 1)
            * descend(&grid, 5, 1)
            * descend(&grid, 7, 1)
            * descend(&grid, 1, 2)
    );
}
