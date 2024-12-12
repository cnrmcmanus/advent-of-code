use utils::*;

fn xmas(matrix: &Matrix<char>, direction: Direction) -> u32 {
    matrix
        .iter()
        .indicies()
        .map(|p| {
            let end: String = (matrix.moves(p, direction.to_point(), false).values())
                .take(3)
                .collect();
            u32::from(matrix[p] == 'X' && end == "MAS")
        })
        .sum()
}

fn crossmas(matrix: &Matrix<char>) -> u32 {
    matrix
        .iter()
        .indicies()
        .filter(|&p| !matrix.is_edge(p))
        .map(|p| {
            let top_pair = (matrix[p + UpLeft], matrix[p + UpRight]);
            let bottom_pair = (matrix[p + DownLeft], matrix[p + DownRight]);
            let left_pair = (matrix[p + UpLeft], matrix[p + DownLeft]);
            let right_pair = (matrix[p + UpRight], matrix[p + DownRight]);
            let check = |pair_1, pair_2| pair_1 == ('S', 'S') && pair_2 == ('M', 'M');
            u32::from(
                matrix[p] == 'A'
                    && (check(bottom_pair, top_pair)
                        || check(top_pair, bottom_pair)
                        || check(left_pair, right_pair)
                        || check(right_pair, left_pair)),
            )
        })
        .sum()
}

pub fn main() {
    let matrix = Matrix::from_stdin();

    let all_xmas: u32 = DIRECTIONS.map(|step| xmas(&matrix, step)).iter().sum();
    println!("{}", all_xmas);

    println!("{}", crossmas(&matrix));
}
