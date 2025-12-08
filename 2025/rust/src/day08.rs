use utils::*;

type Point3D = (u64, u64, u64);

fn parse_point3d(s: &str) -> Point3D {
    let mut parts = s.split(',');
    let a = parts.next().unwrap().parse().unwrap();
    let b = parts.next().unwrap().parse().unwrap();
    let c = parts.next().unwrap().parse().unwrap();
    (a, b, c)
}

fn distance_squared(a: Point3D, b: Point3D) -> i64 {
    let dx = a.0 as i64 - b.0 as i64;
    let dy = a.1 as i64 - b.1 as i64;
    let dz = a.2 as i64 - b.2 as i64;
    dx * dx + dy * dy + dz * dz
}

fn nearest(points: &[Point3D]) -> Vec<(Point3D, Point3D)> {
    let mut pairs = (0..points.len())
        .flat_map(|i| (i + 1..points.len()).map(move |j| (points[i], points[j])))
        .collect_vec();
    pairs.sort_by_key(|a| distance_squared(a.0, a.1));
    pairs
}

fn in_circuit(point: Point3D, circuits: &[Vec<Point3D>]) -> Option<usize> {
    (0..circuits.len()).find(|&i| circuits[i].contains(&point))
}

pub fn main() {
    let points = stdin_lines().map(|line| parse_point3d(&line)).collect_vec();
    let nearest = nearest(&points);

    let mut circuits = vec![];
    for (i, (a, b)) in nearest.into_iter().enumerate() {
        match (in_circuit(a, &circuits), in_circuit(b, &circuits)) {
            (Some(i), Some(j)) if i == j => {}
            (Some(i), Some(j)) => {
                let (small, big) = if i < j { (i, j) } else { (j, i) };
                let other = circuits.remove(big);
                circuits[small].extend(other);
            }
            (Some(i), None) => {
                circuits[i].push(b);
            }
            (None, Some(j)) => {
                circuits[j].push(a);
            }
            (None, None) => {
                circuits.push(vec![a, b]);
            }
        }

        if i == 999 {
            circuits.sort_by_key(|circuit| -(circuit.len() as isize));
            let largest3 = circuits[0].len() * circuits[1].len() * circuits[2].len();
            println!("{}", largest3);
        }

        if circuits.len() == 1 && circuits[0].len() == points.len() {
            println!("{}", a.0 * b.0);
            break;
        }
    }
}
