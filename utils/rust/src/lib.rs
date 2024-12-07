use itertools::Itertools;
use regex::Regex;
use std::{
    io::{BufRead, Read},
    str::FromStr,
};

pub fn stdin<T, F: Fn(String) -> T>(f: F) -> Vec<T> {
    stdin_lines().map(&f).collect()
}

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

pub fn magnitude(n: u64) -> u32 {
    if n == 0 {
        1
    } else {
        n.ilog10() + 1
    }
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
pub type IPoint = (isize, isize);

#[derive(Default)]
pub struct Matrix<T> {
    pub data: Vec<Vec<T>>,
    pub w: usize,
    pub h: usize,
}

impl<T: PartialEq + Copy> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix {
            data: Vec::new(),
            w: 0,
            h: 0,
        }
    }

    pub fn from(data: Vec<Vec<T>>) -> Matrix<T> {
        Matrix {
            h: data.len(),
            w: data.first().map_or(0, Vec::len),
            data,
        }
    }

    pub fn bounds(&self) -> Point {
        (self.h, self.w)
    }

    pub fn indicies(&self) -> MatrixIterator<T> {
        MatrixIterator {
            matrix: self,
            position: (0, 0),
            movement: MatrixIteratorMovement::Increment,
        }
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

    pub fn moves(
        &self,
        from: Point,
        direction: Direction,
        wraps: bool,
    ) -> MatrixIterator<T> {
        MatrixIterator {
            matrix: self,
            position: from,
            movement: MatrixIteratorMovement::Direction { direction, wraps },
        }
    }

    pub fn is_edge(&self, (i, j): Point) -> bool {
        i == 0 || j == 0 || i == self.h - 1 || j == self.w - 1
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

impl<T> std::ops::Index<Point> for Matrix<T> {
    type Output = T;

    fn index(&self, (i, j): Point) -> &T {
        &self.data[i][j]
    }
}

impl<T> std::ops::Index<IPoint> for Matrix<T> {
    type Output = T;

    fn index(&self, (i, j): IPoint) -> &T {
        &self.data[i as usize][j as usize]
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

pub static DIRECTIONS: [Direction; 8] =
    [Up, UpRight, Right, DownRight, Down, DownLeft, Left, UpLeft];

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

impl std::ops::Add<Point> for Direction {
    type Output = Point;

    fn add(self, (i, j): Point) -> Point {
        let (o_i, o_j) = self.to_offset();
        ((i as isize + o_i) as usize, (j as isize + o_j) as usize)
    }
}

impl std::ops::Add<Direction> for Point {
    type Output = IPoint;

    fn add(self, direction: Direction) -> IPoint {
        let (i, j) = self;
        let (o_i, o_j) = direction.to_offset();
        ((i as isize + o_i), (j as isize + o_j))
    }
}

enum MatrixIteratorMovement {
    Increment,
    Direction { direction: Direction, wraps: bool },
}

pub struct MatrixIterator<'a, T> {
    matrix: &'a Matrix<T>,
    position: Point,
    movement: MatrixIteratorMovement,
}

impl<'a, T> MatrixIterator<'a, T> {
    pub fn values(self) -> MatrixIteratorValues<'a, T> {
        MatrixIteratorValues {
            matrix_iterator: self,
        }
    }
}

impl<'a, T: Copy> Iterator for MatrixIterator<'a, T> {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        self.position = match self.movement {
            MatrixIteratorMovement::Direction { direction, wraps } => {
                let (i, j) = self.position + direction;
                let out_bounds =
                    i < 0 || i >= self.matrix.h as isize || j < 0 || j >= self.matrix.w as isize;
                if !wraps && out_bounds {
                    return None;
                }
                (wrap(i, self.matrix.h), wrap(j, self.matrix.w))
            }
            MatrixIteratorMovement::Increment => {
                let row_end = self.position.0 == self.matrix.h - 1;
                let column_end = self.position.1 == self.matrix.w - 1;
                if row_end && column_end {
                    return None;
                }
                if column_end {
                    (self.position.0 + 1, 0)
                } else {
                    (self.position.0, self.position.1 + 1)
                }
            }
        };

        Some(self.position)
    }
}

pub struct MatrixIteratorValues<'a, T> {
    matrix_iterator: MatrixIterator<'a, T>,
}

impl<'a, T: Copy> Iterator for MatrixIteratorValues<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.matrix_iterator
            .next()
            .map(|p| self.matrix_iterator.matrix[p])
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
