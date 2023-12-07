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

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
}

#[derive(PartialEq, Eq)]
struct ProcessedHand {
    hand_type: HandType,
    card_values: Vec<u32>,
}

impl Hand {
    fn parse(input: &str) -> Hand {
        Hand {
            cards: input.chars().collect(),
        }
    }

    fn process(&self, card_value_fn: impl Fn(char) -> u32) -> ProcessedHand {
        let card_values = self.cards.iter().copied().map(card_value_fn).collect();
        let hand_type = compute_hand_type(&card_values);

        ProcessedHand {
            hand_type: hand_type,
            card_values: card_values,
        }
    }
}

fn compute_hand_type(card_values: &Vec<u32>) -> HandType {
    // observation: it's optimal to replace jokers with the most common card
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

    if counts.len() <= 1 {
        HandType::FiveOfAKind
    } else if counts.len() == 2 && counts[0] == 4 {
        HandType::FourOfAKind
    } else if counts.len() == 2 {
        HandType::FullHouse
    } else if counts.len() == 3 && counts[0] == 3 {
        HandType::ThreeOfAKind
    } else if counts.len() == 3 {
        HandType::TwoPair
    } else if counts.len() == 4 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

impl PartialOrd for ProcessedHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => self.card_values.partial_cmp(&other.card_values),
            ord => return ord,
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

    hands_bids.sort_by(|(h1, _), (h2, _)| h1.partial_cmp(&h2).unwrap());
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

    hands_bids.sort_by(|(h1, _), (h2, _)| h1.partial_cmp(&h2).unwrap());
    hands_bids
        .iter()
        .enumerate()
        .map(|(i, (_, b))| (i as u32 + 1) * b)
        .sum()
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", solve1(input));
    println!("{}", solve2(input))
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
