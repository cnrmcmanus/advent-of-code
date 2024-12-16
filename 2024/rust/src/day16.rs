use utils::*;

#[derive(new)]
struct State {
    pos: Point,
    dir: Direction,
    score: u64,
    path: Vec<Point>,
}

fn insert(
    best: u64,
    ends: &mut Vec<State>,
    seen: &mut HashMap<(Point, Direction), u64>,
    mut state: State,
) {
    let worse = (seen.get(&(state.pos, state.dir))).map_or(false, |&best| best < state.score);
    if state.score >= best || worse {
        return;
    }

    seen.insert((state.pos, state.dir), state.score);
    state.path.push(state.pos);
    ends.sorted_insert_by_key(state, |state| state.score);
}

pub fn main() {
    let matrix = Matrix::from_stdin();
    let start = matrix.position('S').unwrap();

    let mut ends = vec![State::new(start, Right, 0, vec![start])];
    let mut seen = HashMap::from([((start, Right), 0)]);
    let mut best = u64::MAX;
    let mut points = HashSet::new();

    while let Some(end) = ends.pop() {
        if matrix[end.pos + end.dir] == 'E' && end.score <= best {
            points.extend(end.path);
            best = end.score;
            continue;
        }

        if matrix[end.pos + end.dir] != '#' {
            let end = State::new(end.pos + end.dir, end.dir, end.score + 1, end.path.clone());
            insert(best, &mut ends, &mut seen, end);
        }

        for turn in [end.dir.rotate(2), end.dir.rotate(4), end.dir.rotate(6)] {
            if matrix[end.pos + turn] != '#' {
                let end = State::new(end.pos, turn, end.score + 1000, end.path.clone());
                insert(best, &mut ends, &mut seen, end);
            }
        }
    }

    println!("{}", best + 1);
    println!("{}", points.len() + 1);
}
