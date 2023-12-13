use crate::util::*;

fn fits(n: usize, line: &[char], i: usize, end: usize, first: bool) -> bool {
    let j = i + n;
    usize::from(!first) + j <= end
        && line[i..j].iter().all(|&c| c != '.')
        && line[j..end].iter().all(|&c| c != '#')
}

fn solve(line: Vec<char>, pattern: Vec<usize>) -> u64 {
    let mut cache: Vec<u64> = vec![0; line.len()];
    let last_k = pattern.len() - 1;
    let last = pattern[pattern.len() - 1];
    for (i, cell) in cache.iter_mut().enumerate().take(line.len() - (last - 1)) {
        *cell = u64::from(fits(last, &line, i, line.len(), true));
    }

    pattern
        .into_iter()
        .rev()
        .enumerate()
        .skip(1)
        .fold(cache, |cache, (k, number)| {
            let mut next_cache = vec![0; line.len()];
            for i in 0..line.len() - (number - 1) {
                if k == last_k && !line[..i].iter().all(|&c| c != '#') {
                    next_cache[i] = 0;
                    continue;
                }
                for (j, &score) in cache.iter().enumerate().skip(number - 1) {
                    if fits(number, &line, i, j, false) {
                        next_cache[i] += score;
                    }
                }
            }
            next_cache
        })
        .into_iter()
        .sum()
}

pub fn main() {
    let (total_1, total_2) = stdin_lines()
        .map(|line| {
            let (left, right) = line.split_at(line.find(' ').unwrap_or(0));
            let right = &right[1..];
            let score_1 = solve(left.chars().collect(), str_to_vec(right));

            let left = format!("{}?{}?{}?{}?{}", left, left, left, left, left);
            let right = format!("{},{},{},{},{}", right, right, right, right, right);
            let score_2 = solve(left.chars().collect(), str_to_vec(&right));

            (score_1, score_2)
        })
        .sum_tuples();

    println!("{}\n{}", total_1, total_2);
}
