use crate::util::*;

type Point = (isize, isize);

fn parse(line: &str) -> ((char, isize), (char, isize)) {
    let chars: Vec<_> = line.chars().collect();
    let n = chars.len();
    let size = line[2..=2 + (n - 13)].parse().unwrap();
    let hex_size = isize::from_str_radix(&line[n - 7..n - 2], 16).unwrap();
    let hex_dir = ['R', 'D', 'L', 'U'][chars[n - 2].to_digit(10).unwrap() as usize];
    ((chars[0], size), (hex_dir, hex_size))
}

fn next(point: Point, dir: char, size: isize) -> Point {
    match dir {
        'D' => (point.0 + size, point.1),
        'U' => (point.0 - size, point.1),
        'R' => (point.0, point.1 + size),
        'L' => (point.0, point.1 - size),
        _ => point,
    }
}

fn area(points: &[Point]) -> isize {
    (0..points.len())
        .map(|i| {
            let j = (i + 1) % points.len();
            points[i].0 * points[j].1 - points[i].1 * points[j].0
        })
        .sum::<isize>()
        .abs()
        / 2
}

pub fn solve<I: Iterator<Item = (char, isize)>>(moves: I) -> isize {
    let mut point: Point = (0, 0);
    let mut points = vec![point];
    let mut exterior = 0;
    for (dir, size) in moves {
        point = next(point, dir, size);
        exterior += size;
        points.push(point);
    }
    area(&points) + (exterior / 2) + 1
}

pub fn main() {
    let input: Vec<_> = stdin_lines().map(|line| parse(&line)).collect();
    println!("{}", solve(input.iter().map(|t| t.0)));
    println!("{}", solve(input.iter().map(|t| t.1)));
}
