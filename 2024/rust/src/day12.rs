use itertools::iproduct;
use utils::*;

fn flood(m: &Matrix<char>, p: Point) -> HashSet<Point> {
    let mut points = HashSet::from([p]);
    let mut steps = vec![p];
    while let Some(point) = steps.pop() {
        for mov in [Up, Down, Left, Right] {
            let next = point + mov;
            if !m.in_bounds(next) || points.contains(&next) || m[p] != m[next] {
                continue;
            }
            points.insert(next);
            steps.push(next);
        }
    }
    points
}

fn perim(points: &HashSet<Point>) -> u64 {
    iproduct!(points.iter(), [Up, Down, Left, Right]).map_sum(|(&p, mov)| {
        u64::from(!points.contains(&(p + mov)))
    })
}

fn trace(mut p: Point, points: &HashSet<Point>, d: Direction, o: Direction) -> Vec<Point> {
    let mut edge = Vec::new();
    loop {
        edge.push(p);
        if !points.contains(&(p + d)) || points.contains(&(p + d + o)) {
            break;
        }
        p = p + d;
    }
    edge
}

fn sides(points: &HashSet<Point>) -> u64 {
    let mut seen = HashSet::new();
    iproduct!(points, [Up, Left]).map_sum(|(&p, mov)| {
        [mov.rotate(2), mov.rotate(-2)].into_iter().map_sum(|out| {
            if seen.contains(&(p, mov, out)) || points.contains(&(p + out)) {
                return 0;
            }

            let inv = mov.rotate(4);
            seen.extend(trace(p, points, mov, out).iter().map(|&p| (p, mov, out)));
            seen.extend(trace(p, points, inv, out).iter().map(|&p| (p, mov, out)));

            1
        })
    })
}

pub fn main() {
    let plots = Matrix::from_stdin();

    let mut seen = HashSet::new();
    let mut perim_score = 0;
    let mut side_score = 0;
    for cell in plots.iter() {
        if seen.contains(&cell.index) {
            continue;
        }
        let points = flood(&plots, cell.index);
        perim_score += points.len() as u64 * perim(&points);
        side_score += points.len() as u64 * sides(&points);
        seen.extend(points);
    }

    println!("{:?}", perim_score);
    println!("{:?}", side_score);
}
