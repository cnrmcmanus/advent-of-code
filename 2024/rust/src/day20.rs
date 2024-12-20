use utils::*;

fn count_paths(
    m: &Matrix<char>,
    beat: usize,
    cheat: usize,
    to_end: &HashMap<Point, usize>,
    to_start: &HashMap<Point, usize>,
) -> usize {
    m.positions('.').map_sum(|cheat_start| {
        m.iter_within_by(cheat_start, cheat, |cell| cell.value != '#')
            .indicies()
            .filter(|&cheat_end| {
                let dist = cheat_start.abs_distance(cheat_end);
                to_start[&cheat_start] + dist + to_end[&cheat_end] < beat
            })
            .count()
    })
}

pub fn main() {
    let mut m = Matrix::from_stdin();
    let start = m.position('S').unwrap();
    let end = m.position('E').unwrap();
    m[start] = '.';
    let to_end = m.all_lowest_costs(end, '#');
    let to_start = m.all_lowest_costs(start, '#');
    let beat = to_end[&start] - 99;
    println!("{}", count_paths(&m, beat, 2, &to_end, &to_start));
    println!("{}", count_paths(&m, beat, 20, &to_end, &to_start));
}
