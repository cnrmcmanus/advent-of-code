use utils::*;

pub fn antenna_pairs(
    antennas: &HashMap<char, Vec<Point>>,
) -> impl Iterator<Item = Vec<Point>> + '_ {
    antennas
        .values()
        .flat_map(|points| points.iter().cloned().combinations(2))
}

pub fn main() {
    let matrix = Matrix::from_stdin();
    let antennas: HashMap<char, Vec<Point>> =
        (matrix.iter().indicies()).fold(HashMap::new(), |mut antennas, i| {
            if matrix[i] != '.' {
                antennas.entry(matrix[i]).or_default().push(i);
            }
            antennas
        });

    let antinodes = antenna_pairs(&antennas)
        .fold(HashSet::new(), |mut seen, pair| {
            let diff = pair[1] - pair[0];
            if matrix.in_bounds(pair[0] - diff) {
                seen.insert(pair[0] - diff);
            }
            if matrix.in_bounds(pair[1] + diff) {
                seen.insert(pair[1] + diff);
            }
            seen
        })
        .len();
    println!("{}", antinodes);

    let antinodes = antenna_pairs(&antennas)
        .fold(HashSet::new(), |mut seen, pair| {
            let diff = (pair[1] - pair[0]).simplify();

            let mut position = pair[0];
            while matrix.in_bounds(position) {
                seen.insert(position);
                position = position - diff;
            }
            position = pair[1];
            while matrix.in_bounds(position) {
                seen.insert(position);
                position = position + diff;
            }
            seen
        })
        .len();
    println!("{}", antinodes);
}
