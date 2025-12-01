use utils::*;

fn turn(pos: i32, by: i32) -> (i32, i32) {
    let passes = (pos + by).div_euclid(100).abs();
    let new_pos = (pos + by).rem_euclid(100);
    let (left_start_zero, left_end_zero) = (pos == 0 && by < 0, new_pos == 0 && by < 0);
    (new_pos, passes - i32::from(left_start_zero) + i32::from(left_end_zero))
}

pub fn main() {
    let mut pos = 50;
    let (naive, click_method) = stdin_lines().map_sum_tuple(|line| {
        let (dir, amount_str) = line.split_at(1);
        let amount = amount_str.parse::<i32>().unwrap();

        let (new_pos, score) = turn(pos, amount * (if dir == "L" { -1 } else { 1 }));
        pos = new_pos;

        (i32::from(pos == 0), score)
    });
    println!("{}\n{}", naive, click_method);
}
