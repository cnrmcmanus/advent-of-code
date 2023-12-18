use crate::util::*;
use std::collections::BinaryHeap;
use std::collections::HashSet;

type Point = (isize, isize);

#[derive(Debug, Eq, PartialEq)]
struct Path {
    point: Point,
    dir: Point,
    cost: u64,
    must_turn: u32,
    cant_turn: u32,
}

impl Path {
    fn new(point: Point, dir: Point, cost: u64, must_turn: u32, cant_turn: u32) -> Path {
        Path {
            point,
            dir,
            cost,
            must_turn,
            cant_turn,
        }
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

fn inside(point: Point, max: Point) -> bool {
    point.0 >= 0 && point.0 < max.0 && point.1 >= 0 && point.1 < max.1
}

fn shortest_path(map: &[Vec<u64>], must_turn: u32, cant_turn: u32) -> u64 {
    let max = (map.len() as isize, map[0].len() as isize);
    let mut queue = BinaryHeap::from([
        Path::new((1, 0), (1, 0), map[1][0], must_turn, cant_turn),
        Path::new((0, 1), (0, 1), map[0][1], must_turn, cant_turn),
    ]);
    let mut seen = HashSet::new();

    while let Some(path) = queue.pop() {
        if path.point.0 == max.0 - 1 && path.point.1 == max.1 - 1 {
            return path.cost;
        }

        if seen.contains(&(path.point, path.dir, path.must_turn, path.cant_turn)) {
            continue;
        }
        seen.insert((path.point, path.dir, path.must_turn, path.cant_turn));

        let l_dir = (-path.dir.1, path.dir.0);
        let l_point = (path.point.0 + l_dir.0, path.point.1 + l_dir.1);
        if inside(l_point, max) && path.cant_turn == 0 {
            let cost = path.cost + map[l_point.0 as usize][l_point.1 as usize];
            queue.push(Path::new(l_point, l_dir, cost, must_turn, cant_turn));
        }

        let r_dir = (path.dir.1, -path.dir.0);
        let r_point = (path.point.0 + r_dir.0, path.point.1 + r_dir.1);
        if inside(r_point, max) && path.cant_turn == 0 {
            let cost = path.cost + map[r_point.0 as usize][r_point.1 as usize];
            queue.push(Path::new(r_point, r_dir, cost, must_turn, cant_turn));
        }

        let s_point = (path.point.0 + path.dir.0, path.point.1 + path.dir.1);
        if inside(s_point, max) && path.must_turn > 0 {
            let cost = path.cost + map[s_point.0 as usize][s_point.1 as usize];
            let must_turn = path.must_turn - 1;
            let cant_turn = path.cant_turn - u32::from(path.cant_turn != 0);
            queue.push(Path::new(s_point, path.dir, cost, must_turn, cant_turn));
        }
    }

    0
}

pub fn main() {
    let map: Vec<Vec<u64>> = stdin_lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect();

    println!("{}", shortest_path(&map, 2, 0));
    println!("{}", shortest_path(&map, 9, 3));
}
