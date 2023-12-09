use crate::util::*;

pub fn main() {
    let mut extra_plays = [0; 256];
    let (total_1, total_2) = stdin_lines()
        .enumerate()
        .map(|(game, line)| {
            let start1 = line.find(':').unwrap_or(0);
            let start2 = line.find('|').unwrap_or(0);
            let winning = str_to_vec::<u32>(&line[start1 + 1..start2 - 1]);
            let hand = str_to_vec::<u32>(&line[start2 + 1..]);
            let plays = extra_plays[game] + 1;

            let (score_1, count) = winning.iter().fold((0, 0), |(score_1, count), number| {
                if hand.contains(number) {
                    (if score_1 == 0 { 1 } else { score_1 * 2 }, count + 1)
                } else {
                    (score_1, count)
                }
            });

            for next_plays in extra_plays.iter_mut().skip(game + 1).take(count) {
                *next_plays += plays;
            }

            (score_1, plays)
        })
        .sum_tuples();

    println!("{}\n{}", total_1, total_2);
}
