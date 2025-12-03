use utils::*;

fn largest_joltage(nums: &[u32], find_n: usize) -> (u64, usize) {
    (0..find_n).fold((0, 0), |(joltage, start_index), i| {
        let range = (start_index..nums.len() + 1 - (find_n - i)).rev();
        let index = range.max_by_key(|&j| &nums[j]).unwrap();
        (joltage * 10 + nums[index] as u64, index + 1)
    })
}

pub fn main() {
    let (joltage_2, joltage_12) = stdin_lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .map_sum_tuple(|nums| (largest_joltage(&nums, 2).0, largest_joltage(&nums, 12).0));
    println!("{}\n{}", joltage_2, joltage_12);
}
