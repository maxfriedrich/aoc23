use std::collections::HashMap;

const JOKER_VALUE: u32 = 1;

fn card_value_1(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!(),
    }
}

fn card_value_2(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => JOKER_VALUE,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!(),
    }
}

fn value_counts<T: std::marker::Copy + std::cmp::Eq + PartialEq + std::hash::Hash>(
    items: &[T],
) -> HashMap<T, usize> {
    items.iter().copied().fold(HashMap::new(), |mut map, val| {
        *map.entry(val).or_default() += 1;
        map
    })
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
}

#[derive(PartialEq, Eq)]
struct ProcessedHand {
    hand_type: Vec<usize>,
    card_values: Vec<u32>,
}

impl Hand {
    fn parse(input: &str) -> Hand {
        Hand {
            cards: input.chars().collect(),
        }
    }

    fn process(&self, card_value_fn: impl Fn(char) -> u32) -> ProcessedHand {
        let card_values: Vec<u32> = self.cards.iter().copied().map(card_value_fn).collect();
        let hand_type = compute_hand_type(&card_values);

        ProcessedHand {
            hand_type,
            card_values,
        }
    }
}

fn compute_hand_type(card_values: &[u32]) -> Vec<usize> {
    // Observation: it's optimal to replace jokers with the most common card

    // No need for a special hand type enum because the hand types have the same order
    // that you can get by sorting the value counts:
    // [5] > [4,1] > [3,2] > [3,1,1] > [2,2,1] > [2,1,1,1] > [1,1,1,1,1]

    let card_to_count = value_counts(card_values);
    let num_jokers = card_to_count.get(&JOKER_VALUE).unwrap_or(&0);

    let mut found_jokers_count = false;
    let mut counts: Vec<usize> = Vec::new();
    for count in card_to_count.values() {
        if count == num_jokers && !found_jokers_count {
            found_jokers_count = true;
            continue;
        }
        counts.push(*count);
    }
    counts.sort();
    counts.reverse();

    if let Some(most_common) = counts.first_mut() {
        *most_common += num_jokers;
    }
    counts
}

impl PartialOrd for ProcessedHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => self.card_values.partial_cmp(&other.card_values),
            ord => ord,
        }
    }
}

fn parse(input: &str) -> (Hand, u32) {
    let (hand_str, bid_str) = input.split_once(' ').unwrap();
    let hand = Hand::parse(hand_str);
    let bid = bid_str.parse().unwrap();
    (hand, bid)
}

fn solve1(input: &str) -> u32 {
    let mut hands_bids: Vec<(ProcessedHand, u32)> = input
        .lines()
        .map(parse)
        .map(|(h, b)| (h.process(card_value_1), b))
        .collect();

    hands_bids.sort_by(|(h1, _), (h2, _)| h1.partial_cmp(h2).unwrap());
    hands_bids
        .iter()
        .enumerate()
        .map(|(i, (_, b))| (i as u32 + 1) * b)
        .sum()
}

fn solve2(input: &str) -> u32 {
    let mut hands_bids: Vec<(ProcessedHand, u32)> = input
        .lines()
        .map(parse)
        .map(|(h, b)| (h.process(card_value_2), b))
        .collect();

    hands_bids.sort_by(|(h1, _), (h2, _)| h1.partial_cmp(h2).unwrap());
    hands_bids
        .iter()
        .enumerate()
        .map(|(i, (_, b))| (i as u32 + 1) * b)
        .sum()
}

fn main() {
    use aoc::Timer;
    let input = include_str!("input.txt");

    let timer = Timer::new();
    let result1 = solve1(input);
    println!("Part 1: {} ({}ms)", result1, timer.elapsed().as_millis());

    let timer = Timer::new();
    let result2 = solve2(input);
    println!("Part 2: {} ({}ms)", result2, timer.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 6440);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 5905);
    }
}
