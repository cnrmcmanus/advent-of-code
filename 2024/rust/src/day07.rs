use utils::*;

fn solvable(goal: u64, use_concat: bool, sub_total: u64, nums: &[u64]) -> bool {
    if nums.is_empty() {
        return goal == sub_total;
    }

    let n = nums[0];
    let add = sub_total + n;
    let mul = sub_total * n;
    let concat = sub_total * 10u64.pow(magnitude(n)) + n;

    (add <= goal && solvable(goal, use_concat, add, &nums[1..]))
        || (mul <= goal && solvable(goal, use_concat, mul, &nums[1..]))
        || (use_concat && concat <= goal && solvable(goal, use_concat, concat, &nums[1..]))
}

pub fn main() {
    let equations: Vec<(u64, Vec<u64>)> = stdin(|line| {
        let mut parts = line.split(": ");
        let goal = parts.next().unwrap().parse().unwrap();
        let nums = parts.next().unwrap().split_and_parse(" ").collect_vec();
        (goal, nums)
    });

    for use_concat in [false, true] {
        let total: u64 = equations
            .iter()
            .map(|(goal, nums)| u64::from(solvable(*goal, use_concat, 0, nums)) * goal)
            .sum();
        println!("{}", total);
    }
}
