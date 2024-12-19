use utils::*;

#[memoize]
fn dfs(towels: Vec<String>, design: String) -> u64 {
    if design.is_empty() {
        return 1;
    }

    towels.iter().map_sum(|towel| {
        if design.starts_with(towel) {
            dfs(towels.clone(), design[towel.len()..].to_string())
        } else {
            0
        }
    })
}

pub fn main() {
    let mut input = stdin_lines();
    let towels = (input.next().unwrap())
        .split_and_parse::<String>(", ")
        .collect_vec();
    input.next();
    let designs = input.collect_vec();

    let (valid, combos) = designs.iter().map_sum_tuple(|design| {
        let n = dfs(towels.clone(), design.clone());
        (u64::from(n != 0), n)
    });

    println!("{}", valid);
    println!("{}", combos);
}
