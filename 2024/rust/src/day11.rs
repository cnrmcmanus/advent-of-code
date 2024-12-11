use utils::*;

fn blink(stones: &mut HashMap<u64, u64>) {
    let mut next = HashMap::new();

    for (&n, &t) in stones.iter() {
        let m = magnitude(n) as u64;

        if n == 0 {
            *next.entry(1).or_insert(0) += t;
        } else if m % 2 == 0 {
            let h = 10u64.pow(m as u32 / 2);
            *next.entry(n / h).or_insert(0) += t;
            *next.entry(n % h).or_insert(0) += t;
        } else {
            let x = n * 2024;
            *next.entry(x).or_insert(0) += t;
        }
    }

    *stones = next;
}

fn size(stones: &HashMap<u64, u64>) -> u64 {
    stones.iter().fold(0, |acc, (_, t)| acc + t)
}

pub fn main() {
    let mut stones = HashMap::new();
    for stone in stdin_all().split_and_parse(" ") {
        stones.insert(stone, 1);
    }

    (0..25).for_each(|_| blink(&mut stones));
    println!("{}", size(&stones));

    (0..50).for_each(|_| blink(&mut stones));
    println!("{}", size(&stones));
}
