use crate::util::*;
use std::collections::HashMap;

type Card = u32;

fn to_hand(cards: &[Card], joker_mode: bool) -> Vec<u32> {
    let mut groups = HashMap::new();
    for &card in cards {
        *groups.entry(card).or_insert(0) += 1;
    }
    let jokers = if joker_mode {
        groups.remove(&11).unwrap_or(0)
    } else {
        0
    };

    let mut counts = groups.into_values().collect::<Vec<_>>();
    counts.sort();
    counts.reverse();

    match counts.get_mut(0) {
        Some(head) => *head += jokers,
        None => counts.push(5),
    }

    counts
}

fn parse(c: char) -> Card {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        n => n.to_digit(10).unwrap_or(0),
    }
}

fn winnings(input: &[([Card; 5], u32)], joker_mode: bool) -> u32 {
    let mut hands: Vec<_> = input
        .iter()
        .map(|(cards, bid)| {
            let hand = to_hand(cards, joker_mode);
            let cards = cards.map(|card| if joker_mode && card == 11 { 0 } else { card });
            ((hand, cards), bid)
        })
        .collect();

    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank as u32 + 1) * bid)
        .sum()
}

pub fn main() {
    let lines = stdin_lines();
    let input: Vec<_> = lines
        .map(|line| {
            let (left, right) = line.split_at(6);
            let cards: Vec<_> = left.chars().take(5).map(parse).collect();
            let bid = right.parse().unwrap_or(0);
            (cards.try_into().unwrap_or_default(), bid)
        })
        .collect();

    println!("{}\n{}", winnings(&input, false), winnings(&input, true));
}
