use utils::*;

fn tick(mut secret: i64) -> i64 {
    secret ^= secret << 6 & 0xFFFFFF;
    secret ^= secret >> 5;
    secret ^ secret << 11 & 0xFFFFFF
}

pub fn main() {
    let secrets: Vec<Vec<i64>> = stdin(|num| {
        successors(Some(num.parse().unwrap()), |&secret| Some(tick(secret)))
            .map(|n| n % 10)
            .take(2000 + 1)
            .collect()
    });

    println!("{}", secrets.iter().map_sum(|ticks| ticks[2000]));

    let mut cache = HashMap::new();
    secrets.into_iter().for_each(|ticks| {
        (ticks.iter().tuple_windows())
            .map(|(&a, &b)| b % 10 - a % 10)
            .tuple_windows::<(i64, i64, i64, i64)>()
            .enumerate()
            .unique_by(|(_, seq)| *seq)
            .for_each(|(i, seq)| *cache.entry(seq).or_insert(0) += ticks[i + 4]);
    });
    println!("{}", cache.values().max().unwrap());
}
