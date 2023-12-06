use crate::util::*;

fn glob_number(line: &str) -> u64 {
    line.chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

fn score(time: u64, record: u64) -> u64 {
    (1..time).fold(0, |score, hold| {
        let distance = (time - hold) * hold;
        if distance > record {
            score + 1
        } else {
            score
        }
    })
}

pub fn main() {
    let mut lines = stdin_lines();
    let times_line = lines.next().unwrap();
    let distances_line = lines.next().unwrap();

    let times = str_to_vec(&times_line);
    let records = str_to_vec(&times_line);
    let score_part1 = times
        .into_iter()
        .zip(records)
        .fold(1, |total, (time, record)| total * score(time, record));
    println!("{}", score_part1);

    let big_time = glob_number(&times_line);
    let big_record = glob_number(&distances_line);
    println!("{}", score(big_time, big_record));
}
