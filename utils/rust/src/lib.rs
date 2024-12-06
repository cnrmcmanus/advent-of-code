use itertools::Itertools;
use regex::Regex;
use std::{
    io::{BufRead, Read},
    str::FromStr,
};

pub fn stdin_lines() -> impl Iterator<Item = String> {
    std::io::stdin().lock().lines().map_while(Result::ok)
}

pub fn stdin_all() -> String {
    let mut contents = String::new();
    std::io::stdin().lock().read_to_string(&mut contents).ok();
    contents
}

pub fn stdin_lines_by(pattern: &str) -> Vec<String> {
    let re = Regex::new(pattern).unwrap();
    re.split(&stdin_all()).map(ToOwned::to_owned).collect()
}

pub fn regex(re: &str) -> Regex {
    Regex::new(re).expect("failed to parse regex")
}

pub fn wrap(index: isize, max: usize) -> usize {
    let max = max as isize;
    (((index % max) + max) % max) as usize
}

pub trait StrExtensions {
    fn parse_ok<F: FromStr>(&self) -> Option<F>;

    fn split_and_parse<T: FromStr>(&self, delim: &str) -> impl Iterator<Item = T>;
}

impl StrExtensions for str {
    fn parse_ok<F: FromStr>(&self) -> Option<F> {
        self.parse::<F>().ok()
    }

    fn split_and_parse<T: FromStr>(&self, delim: &str) -> impl Iterator<Item = T> {
        self.split(delim).filter_map(str::parse_ok)
    }
}

pub type Point = (usize, usize);

#[derive(Default)]
pub struct Matrix<T> {
    pub data: Vec<Vec<T>>,
}

impl<T: PartialEq + Copy> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix { data: Vec::new() }
    }

    pub fn from(data: Vec<Vec<T>>) -> Matrix<T> {
        Matrix { data }
    }

    pub fn dims(&self) -> (usize, usize) {
        (
            self.data.len(),
            self.data.first().map_or_else(|| 0, Vec::len),
        )
    }

    pub fn indicies(&self) -> impl Iterator<Item = (usize, usize)> {
        let (h, w) = self.dims();
        (0..h).cartesian_product(0..w)
    }

    pub fn map<U, F>(&self, f: F) -> Matrix<U>
    where
        U: PartialEq + Copy,
        F: Fn(&T, (usize, usize)) -> U,
    {
        let data = (self.data.iter().enumerate())
            .map(|(i, row)| row.iter().enumerate().map(|(j, x)| f(x, (i, j))).collect())
            .collect();
        Matrix::from(data)
    }

    pub fn moves(&self, from: (usize, usize), direction: Direction, wraps: bool) -> MatrixIterator {
        MatrixIterator {
            position: from,
            bounds: self.dims(),
            direction,
            wraps,
        }
    }

    pub fn position(&self, x: T) -> Option<(usize, usize)> {
        self.data
            .iter()
            .enumerate()
            .find_map(|(i, row)| row.iter().position(|&y| y == x).map(|j| (i, j)))
    }
}

impl Matrix<char> {
    pub fn from_stdin() -> Matrix<char> {
        let data = stdin_lines().map(|line| line.chars().collect()).collect();
        Matrix::from(data)
    }

    pub fn print(&self) {
        for row in self.data.iter() {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

pub use Direction::*;

static DIRECTIONS: [Direction; 8] = [Up, UpRight, Right, DownRight, Down, DownLeft, Left, UpLeft];

impl Direction {
    pub fn rotate(&self, by: isize) -> Direction {
        let i = DIRECTIONS.iter().position(|d| d == self).unwrap() as isize;
        DIRECTIONS[wrap(i + by, DIRECTIONS.len())]
    }

    pub fn to_offset(&self) -> (isize, isize) {
        match self {
            Up => (-1, 0),
            UpRight => (-1, 1),
            Right => (0, 1),
            DownRight => (1, 1),
            Down => (1, 0),
            DownLeft => (1, -1),
            Left => (0, -1),
            UpLeft => (-1, -1),
        }
    }
}

pub struct MatrixIterator {
    position: (usize, usize),
    bounds: (usize, usize),
    direction: Direction,
    wraps: bool,
}

impl Iterator for MatrixIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.direction.to_offset();
        let i = self.position.0 as isize + offset.0;
        let j = self.position.1 as isize + offset.1;

        if !self.wraps
            && (i < 0 || i >= self.bounds.0 as isize || j < 0 || j >= self.bounds.1 as isize)
        {
            return None;
        }

        self.position = (wrap(i, self.bounds.0), wrap(j, self.bounds.1));
        Some(self.position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_rotates_clockwise() {
        assert_eq!(DownRight.rotate(2), DownLeft);
        assert_eq!(DownRight.rotate(5), Up);
        assert_eq!(DownRight.rotate(8), DownRight);
        assert_eq!(DownRight.rotate(17), Down);
    }

    #[test]
    fn direction_rotates_counter_clockwise() {
        assert_eq!(DownRight.rotate(-2), UpRight);
        assert_eq!(DownRight.rotate(-4), UpLeft);
        assert_eq!(DownRight.rotate(-8), DownRight);
        assert_eq!(DownRight.rotate(-17), Right);
    }
}
