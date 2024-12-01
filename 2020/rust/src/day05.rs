use utils::*;

type URange = (usize, usize);

fn subdivide(((il, iu), (jl, ju)): (URange, URange), region: char) -> (URange, URange) {
    let ih = (iu - il) / 2;
    let jh = (ju - jl) / 2;
    match region {
        'F' => ((il, iu), (jl, jl + jh)),
        'B' => ((il, iu), (ju - jh, ju)),
        'L' => ((il, il + ih), (jl, ju)),
        'R' => ((iu - ih, iu), (jl, ju)),
        _ => ((il, iu), (jl, ju)),
    }
}

fn find_seat(boarding_pass: &str) -> usize {
    let ((i, _), (j, _)) = boarding_pass.chars().fold(((0, 8), (0, 128)), subdivide);
    (j * 8) + i
}

pub fn main() {
    let mut seat_ids: Vec<_> = stdin_lines().map(|pass| find_seat(&pass)).collect();
    seat_ids.sort();
    let max_seat_id = seat_ids.last().cloned().unwrap_or_default();
    let packed = seat_ids
        .iter()
        .skip_while(|&&id| id < 8)
        .take_while(|&&id| id < max_seat_id - (max_seat_id % 8))
        .cloned();
    println!("{:?}", max_seat_id);
    println!(
        "{:?}",
        packed
            .clone()
            .zip(packed.skip(1))
            .find(|&(i, j)| i + 1 != j)
            .unwrap_or_default()
            .0
            + 1
    );
}
