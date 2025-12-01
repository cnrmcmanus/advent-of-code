use utils::*;

fn rotate(pos: u32, dir: &str, amount: u32) -> (u32, u32) {
    let pos_rotated = if dir == "L" { 100 - pos } else { pos } + amount;
    let score = (pos_rotated / 100) - u32::from(pos == 0 && dir == "L");
    let new_pos = pos_rotated % 100;
    let new_pos = if dir == "L" { 100 - new_pos } else { new_pos } % 100;
    (new_pos, score)
}

pub fn main() {
    let mut pos = 50;
    let (naive, click_method) = stdin_lines().map_sum_tuple(|line| {
        let (dir, amount_str) = line.split_at(1);
        let amount = amount_str.parse::<u32>().unwrap();

        let (new_pos, score) = rotate(pos, dir, amount);
        pos = new_pos;

        (u32::from(pos == 0), score)
    });
    println!("{}\n{}", naive, click_method);
}
