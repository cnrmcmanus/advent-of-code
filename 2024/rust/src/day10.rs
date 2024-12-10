use utils::*;

fn walk(m: &Matrix<u32>, trailhead: Point, unique_path: bool) -> u32 {
    let mut ends: Vec<Point> = vec![trailhead];
    let mut seen: HashSet<Point> = HashSet::from([]);
    let mut score = 0;

    while let Some(point) = ends.pop() {
        if !unique_path && seen.contains(&point) {
            continue;
        }
        seen.insert(point);

        if m[point] == 9 {
            score += 1;
        }

        for direction in [Up, Down, Left, Right] {
            let next = point + direction;
            if m.in_bounds(next) && m[point] < m[next] && m[next] - m[point] == 1 {
                ends.push(next);
            }
        }
    }

    score
}

pub fn main() {
    let matrix = Matrix::from_stdin().map(|cell| cell.value.to_digit(10).unwrap());

    for unique_path in [false, true] {
        let score: u32 = (matrix.positions(0))
            .map(|trailhead| walk(&matrix, trailhead, unique_path))
            .sum();
        println!("{}", score);
    }
}
