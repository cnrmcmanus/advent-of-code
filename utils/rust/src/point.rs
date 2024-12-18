use num::integer::*;
use std::ops::{Add, Div, Sub};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub i: isize,
    pub j: isize,
}

impl Point {
    pub fn new(i: isize, j: isize) -> Point {
        Point { i, j }
    }

    pub fn tuple((i, j): (isize, isize)) -> Point {
        Point::new(i, j)
    }

    pub fn origin() -> Point {
        Point::new(0, 0)
    }

    pub fn to_tuple(&self) -> (isize, isize) {
        (self.i, self.j)
    }

    pub fn ui(&self) -> usize {
        self.i as usize
    }

    pub fn uj(&self) -> usize {
        self.j as usize
    }

    pub fn unit(&self) -> Point {
        Point::new(self.i.signum(), self.j.signum())
    }

    pub fn simplify(&self) -> Point {
        let d = self.i.gcd(&self.j);
        self / d
    }

    pub fn within(&self, top_left: Point, bottom_right: Point) -> bool {
        self.i >= top_left.i
            && self.i <= bottom_right.i
            && self.j >= top_left.j
            && self.j <= bottom_right.j
    }
}

macro_rules! op {
    ($op:ident, $fn:ident, $a:ty, $b:ty, $body:expr) => {
        impl $op<$b> for $a {
            type Output = Point;

            fn $fn(self, other: $b) -> Point {
                $body(self, other)
            }
        }
    };
}

op!(Add, add, Point, Point, |a: Point, b: Point| Point::new(
    a.i + b.i,
    a.j + b.j
));
op!(Add, add, &Point, Point, |a: &Point, b: Point| Point::new(
    a.i + b.i,
    a.j + b.j
));
op!(Add, add, &Point, &Point, |a: &Point, b: &Point| Point::new(
    a.i + b.i,
    a.j + b.j
));
op!(Sub, sub, Point, Point, |a: Point, b: Point| Point::new(
    a.i - b.i,
    a.j - b.j
));
op!(Sub, sub, &Point, Point, |a: &Point, b: Point| Point::new(
    a.i - b.i,
    a.j - b.j
));
op!(Sub, sub, &Point, &Point, |a: &Point, b: &Point| Point::new(
    a.i - b.i,
    a.j - b.j
));
op!(Div, div, Point, isize, |a: Point, d: isize| Point::new(
    a.i / d,
    a.j / d
));
op!(Div, div, &Point, isize, |a: &Point, d: isize| Point::new(
    a.i / d,
    a.j / d
));

impl From<(usize, usize)> for Point {
    fn from((i, j): (usize, usize)) -> Point {
        Point::new(i as isize, j as isize)
    }
}

impl From<(isize, isize)> for Point {
    fn from((i, j): (isize, isize)) -> Point {
        Point::new(i, j)
    }
}

impl From<(i32, i32)> for Point {
    fn from((i, j): (i32, i32)) -> Point {
        Point::new(i as isize, j as isize)
    }
}
