use std::collections::HashSet;
use utils::*;

pub fn run_patrol(matrix: &Matrix<char>, start: Point, visits: bool) -> (bool, HashSet<Point>) {
    let mut position = start;
    let mut direction = Up;
    let mut visited = HashSet::from([position]);
    let mut collisions = HashSet::new();
    loop {
        let moves = matrix.moves(position, direction, false).collect_vec();
        let pieces = moves.iter().map(|&(i, j)| matrix.data[i][j]).collect_vec();

        if !pieces.contains(&'#') {
            visits.then(|| visited.extend(moves.iter()));
            return (true, visited);
        }

        let pieces = pieces.iter().take_while(|&&p| p != '#');
        let moves = moves.into_iter().take(pieces.count()).collect_vec();
        position = *moves.last().unwrap_or(&position);
        let collision = (direction, position);

        if collisions.contains(&collision) {
            return (false, visited);
        }

        collisions.insert(collision);
        direction = direction.rotate(2);
        visits.then(|| visited.extend(moves.iter()));
    }
}

pub fn main() {
    let matrix: Matrix<char> = Matrix::from_stdin();
    let start = matrix.position('^').unwrap();

    let (_, path) = run_patrol(&matrix, start, true);
    println!("{}", path.len());

    let loops = (path.into_iter().filter(|&p| p != start))
        .map(|block| {
            let matrix = matrix.map(|&x, p| if p == block { '#' } else { x });
            u32::from(!run_patrol(&matrix, start, false).0)
        })
        .sum::<u32>();
    println!("{}", loops);
}
