use utils::*;

pub fn main() {
    let bytes: Vec<Point> =
        stdin(|line| Point::tuple(line.split_and_parse(",").collect_tuple().unwrap()));
    let mut m = Matrix::from(vec![vec!['.'; 71]; 71]);
    (0..1024).for_each(|i| m[bytes[i]] = '#');

    println!("{}", shortest(&m).unwrap());

    for i in 1024..bytes.len() {
        m[bytes[i]] = '#';
        if shortest(&m).is_none() {
            println!("{},{}", bytes[i].i, bytes[i].j);
            break;
        }
    }
}

fn shortest(m: &Matrix<char>) -> Option<usize> {
    let mut ends: Vec<Point> = vec![Point::origin()];
    let mut seen: HashSet<Point> = HashSet::from([Point::origin()]);
    let mut time = 0;

    while !ends.is_empty() {
        let mut next_ends = Vec::with_capacity(1024);
        for pos in ends {
            if pos == m.bottom_right() {
                return Some(time);
            }
            for dir in [Up, Down, Left, Right] {
                let next = pos + dir;
                if m.in_bounds(next) && m[next] != '#' && !seen.contains(&next) {
                    seen.insert(next);
                    next_ends.push(next);
                }
            }
        }
        time += 1;
        ends = next_ends;
    }

    None
}
