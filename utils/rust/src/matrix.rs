use crate::*;

#[derive(Clone, Default)]
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
        Point::from((self.h, self.w))
    }

    pub fn bottom_right(&self) -> Point {
        let i = if self.h == 0 { 0 } else { self.h - 1 };
        let j = if self.w == 0 { 0 } else { self.w - 1 };
        Point::new(i as isize, j as isize)
    }

    pub fn iter(&self) -> MatrixIterator<T> {
        MatrixIterator {
            matrix: self,
            position: Point::origin(),
            movement: MatrixIteratorMovement::Increment,
        }
    }

    pub fn iter_within(&self, center: Point, distance: usize) -> MatrixIterator<T> {
        let iter = center
            .iter_within(distance)
            .filter(move |&p| self.in_bounds(p));
        MatrixIterator {
            matrix: self,
            position: Point::origin(),
            movement: MatrixIteratorMovement::Iterator {
                iter: Box::new(iter),
            },
        }
    }

    pub fn iter_within_by<'a, F>(
        &'a self,
        center: Point,
        distance: usize,
        f: F,
    ) -> MatrixIterator<'a, T>
    where
        F: Fn(Cell<T>) -> bool + 'a,
    {
        let iter = center
            .iter_within(distance)
            .filter(move |&p| self.in_bounds(p) && f(Cell::new(p, self[p])));
        MatrixIterator {
            matrix: self,
            position: Point::origin(),
            movement: MatrixIteratorMovement::Iterator {
                iter: Box::new(iter),
            },
        }
    }

    pub fn map<U, F>(&self, f: F) -> Matrix<U>
    where
        U: PartialEq + Copy,
        F: Fn(Cell<T>) -> U,
    {
        let data = (self.data.iter().enumerate())
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, x)| {
                        f(Cell {
                            value: *x,
                            index: Point::from((i, j)),
                        })
                    })
                    .collect()
            })
            .collect();
        Matrix::from(data)
    }

    pub fn map_cell<F>(&self, index: Point, f: F) -> Matrix<T>
    where
        F: Fn(T) -> T,
    {
        self.map(|cell| {
            (cell.index == index)
                .then(|| f(cell.value))
                .unwrap_or(cell.value)
        })
    }

    pub fn moves(&self, from: Point, direction: Point, wraps: bool) -> MatrixIterator<T> {
        MatrixIterator {
            matrix: self,
            position: from,
            movement: MatrixIteratorMovement::Direction { direction, wraps },
        }
    }

    pub fn is_edge(&self, p: Point) -> bool {
        p.i == 0 || p.j == 0 || p.i == self.h as isize - 1 || p.j == self.w as isize - 1
    }

    pub fn in_bounds(&self, p: Point) -> bool {
        p.within(Point::origin(), self.bottom_right())
    }

    pub fn position(&self, x: T) -> Option<Point> {
        self.iter()
            .find(|cell| cell.value == x)
            .map(|cell| cell.index)
    }

    pub fn positions(&self, x: T) -> impl Iterator<Item = Point> + '_ {
        self.iter()
            .filter(move |cell| cell.value == x)
            .map(|cell| cell.index)
    }

    pub fn positions_by<'a, F>(&'a self, f: F) -> impl Iterator<Item = Point> + 'a
    where
        F: Fn(T) -> bool + 'a,
    {
        self.iter()
            .filter(move |cell| f(cell.value))
            .map(|cell| cell.index)
    }

    pub fn all_lowest_costs(&self, dest: Point, wall: T) -> HashMap<Point, usize> {
        self.all_lowest_costs_by(dest, |to_cell, _, _| to_cell.value != wall)
    }

    pub fn all_lowest_costs_by<F>(&self, dest: Point, f: F) -> HashMap<Point, usize>
    where
        F: Fn(Cell<T>, Cell<T>, Direction) -> bool,
    {
        let mut shortest = HashMap::new();
        let mut steps = HashSet::from([dest]);
        let mut n = 0;
        while !steps.is_empty() {
            let mut next_steps = HashSet::new();
            for step in steps {
                shortest.insert(step, n);
                for dir in [Up, Down, Left, Right] {
                    let next = step + dir;
                    let from_cell = Cell {
                        value: self[step],
                        index: step,
                    };
                    let to_cell = Cell {
                        value: self[next],
                        index: next,
                    };
                    if f(to_cell, from_cell, dir) && !shortest.contains_key(&next) {
                        next_steps.insert(next);
                    }
                }
            }
            steps = next_steps;
            n += 1;
        }
        shortest
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
        println!();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Cell<T: Copy> {
    pub index: Point,
    pub value: T,
}

impl<T: Copy> Cell<T> {
    pub fn new(index: Point, value: T) -> Cell<T> {
        Cell { index, value }
    }
}

impl<T> std::ops::Index<Point> for Matrix<T> {
    type Output = T;

    fn index(&self, p: Point) -> &T {
        &self.data[p.i as usize][p.j as usize]
    }
}

impl<T> std::ops::Index<&Point> for Matrix<T> {
    type Output = T;

    fn index(&self, p: &Point) -> &T {
        &self[*p]
    }
}

impl<T> std::ops::IndexMut<Point> for Matrix<T> {
    fn index_mut(&mut self, p: Point) -> &mut T {
        &mut self.data[p.i as usize][p.j as usize]
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

    pub fn to_point(&self) -> Point {
        Point::from(match self {
            Up => (-1, 0),
            UpRight => (-1, 1),
            Right => (0, 1),
            DownRight => (1, 1),
            Down => (1, 0),
            DownLeft => (1, -1),
            Left => (0, -1),
            UpLeft => (-1, -1),
        })
    }
}

impl std::ops::Add<Point> for Direction {
    type Output = Point;

    fn add(self, p: Point) -> Point {
        p + self.to_point()
    }
}

impl std::ops::Add<Direction> for Point {
    type Output = Point;

    fn add(self, direction: Direction) -> Point {
        direction.to_point() + self
    }
}

enum MatrixIteratorMovement<'a> {
    Increment,
    Direction {
        direction: Point,
        wraps: bool,
    },
    Iterator {
        iter: Box<dyn Iterator<Item = Point> + 'a>,
    },
}

pub struct MatrixIterator<'a, T: PartialEq + Copy> {
    matrix: &'a Matrix<T>,
    position: Point,
    movement: MatrixIteratorMovement<'a>,
}

impl<'a, T: PartialEq + Copy> MatrixIterator<'a, T> {
    pub fn values(self) -> MatrixIteratorValues<'a, T> {
        MatrixIteratorValues {
            matrix_iterator: self,
        }
    }

    pub fn indicies(self) -> MatrixIteratorIndicies<'a, T> {
        MatrixIteratorIndicies {
            matrix_iterator: self,
        }
    }
}

impl<'a, T: PartialEq + Copy> Iterator for MatrixIterator<'a, T> {
    type Item = Cell<T>;

    fn next(&mut self) -> Option<Cell<T>> {
        self.position = match &mut self.movement {
            MatrixIteratorMovement::Direction { direction, wraps } => {
                let p = self.position + *direction;
                if !*wraps && !self.matrix.in_bounds(p) {
                    return None;
                }
                Point::from((wrap(p.i, self.matrix.h), wrap(p.j, self.matrix.w)))
            }
            MatrixIteratorMovement::Increment => {
                let row_end = self.position.ui() == self.matrix.h - 1;
                let column_end = self.position.uj() == self.matrix.w - 1;
                if row_end && column_end {
                    return None;
                }
                Point::from(if column_end {
                    (self.position.i + 1, 0)
                } else {
                    (self.position.i, self.position.j + 1)
                })
            }
            MatrixIteratorMovement::Iterator { iter } => {
                let point = iter.next()?;
                if !self.matrix.in_bounds(point) {
                    return None;
                }
                point
            }
        };

        Some(Cell {
            index: self.position,
            value: self.matrix[self.position],
        })
    }
}

pub struct MatrixIteratorValues<'a, T: PartialEq + Copy> {
    matrix_iterator: MatrixIterator<'a, T>,
}

impl<'a, T: PartialEq + Copy> Iterator for MatrixIteratorValues<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.matrix_iterator
            .next()
            .map(|cell| self.matrix_iterator.matrix[cell.index])
    }
}

pub struct MatrixIteratorIndicies<'a, T: PartialEq + Copy> {
    matrix_iterator: MatrixIterator<'a, T>,
}

impl<'a, T: PartialEq + Copy> Iterator for MatrixIteratorIndicies<'a, T> {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        self.matrix_iterator.next().map(|cell| cell.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix() -> Matrix<u32> {
        Matrix::from(vec![
            vec![0, 1, 2, 3, 4],
            vec![5, 6, 7, 8, 9],
            vec![10, 11, 12, 13, 14],
            vec![15, 16, 17, 18, 19],
        ])
    }

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

    #[test]
    fn point_adds_direction() {
        assert_eq!(Point::new(3, 1) + DownRight, Point::new(4, 2));
        assert_eq!(Point::new(5, 2) + Left, Point::new(5, 1));
        assert_eq!(Point::new(7, 9) + UpLeft, Point::new(6, 8));
    }

    #[test]
    fn matrix_moves_bounded() {
        let matrix = matrix();
        assert_eq!(
            matrix
                .moves(Point::new(1, 2), DownRight.to_point(), false)
                .values()
                .collect_vec(),
            vec![13, 19]
        )
    }

    #[test]
    fn matrix_moves_unbounded() {
        let matrix = matrix();
        assert_eq!(
            matrix
                .moves(Point::new(1, 2), DownRight.to_point(), true)
                .values()
                .take(4)
                .collect_vec(),
            vec![13, 19, 0, 6]
        )
    }
}
