use std::collections::HashMap;

type Card = char;

const CARDS_NO_JOKER: &str = "AKQT98765432";

fn card_value_1(card: Card) -> u32 {
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

fn card_value_2(card: Card) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
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

#[derive(Debug, PartialEq)]
struct Hand {
    cards: Vec<Card>,
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

impl Hand {
    fn parse(input: &str) -> Hand {
        Hand {
            cards: input.chars().collect(),
        }
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

fn hand_type(hand: &Hand) -> HandType {
    let card_value_counts = value_counts(&hand.cards);
    let card_counts: Vec<usize> = card_value_counts.values().copied().collect();

    let num_distinct_cards = card_counts.len();

    if num_distinct_cards == 1 {
        HandType::FiveOfAKind
    } else if num_distinct_cards == 2 && card_counts.contains(&4) {
        HandType::FourOfAKind
    } else if num_distinct_cards == 2 {
        HandType::FullHouse
    } else if num_distinct_cards == 3 && card_counts.contains(&3) {
        HandType::ThreeOfAKind
    } else if num_distinct_cards == 3 {
        HandType::TwoPair
    } else if num_distinct_cards == 4 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

fn hand_type_2(hand: &Hand) -> HandType {
    let joker_positions: Vec<usize> = hand
        .cards
        .iter()
        .enumerate()
        .filter(|(_, c)| *c == &'J')
        .map(|(i, _)| i)
        .collect();

    let num_jokers = joker_positions.len();

    if num_jokers == 0 {
        return hand_type(hand);
    }

    // cover the most expensive cases
    if num_jokers == 5 || num_jokers == 4 {
        return HandType::FiveOfAKind;
    }

    let cards_no_jokers: Vec<Card> = hand.cards.iter().copied().filter(|c| c != &'J').collect();

    let card_value_counts = value_counts(&cards_no_jokers);
    let card_counts: Vec<usize> = card_value_counts.values().copied().collect();
    let num_distinct_cards = card_counts.len();

    if num_jokers == 3 && num_distinct_cards == 1 {
        return HandType::FiveOfAKind;
    }
    if num_jokers == 3 {
        return HandType::FourOfAKind;
    }

    let mut i = cards_no_jokers.len();
    let mut combinations = vec![cards_no_jokers];

    // this is still pretty inefficient, we prune duplicate combinations
    // and/or memoize hand types of each ordered input
    while i < 5 {
        let mut new_combinations = Vec::new();
        for first_cards in combinations {
            for alternative in CARDS_NO_JOKER.chars() {
                let mut new_cards = first_cards.clone();
                new_cards.push(alternative);
                new_combinations.push(new_cards);
            }
        }
        combinations = new_combinations;
        i += 1;
    }

    // dbg!(&combinations.len());

    combinations
        .iter()
        .map(|cards| {
            hand_type(&Hand {
                cards: cards.to_vec(),
            })
        })
        .max()
        .unwrap()
}

fn hand_ordering(
    hand: &Hand,
    other: &Hand,
    hand_type_fn: impl Fn(&Hand) -> HandType,
    card_value_fn: impl Fn(char) -> u32,
) -> Option<std::cmp::Ordering> {
    match hand_type_fn(hand).partial_cmp(&hand_type_fn(other)) {
        Some(core::cmp::Ordering::Equal) => {
            let hand_values: Vec<u32> =
                hand.cards.iter().map(|card| card_value_fn(*card)).collect();
            let other_values: Vec<u32> = other
                .cards
                .iter()
                .map(|card| card_value_fn(*card))
                .collect();
            hand_values.partial_cmp(&other_values)
        }
        ord => ord,
    }
}

#[derive(Debug)]
struct HandBid {
    hand: Hand,
    bid: u32,
}

impl HandBid {
    fn parse(input: &str) -> HandBid {
        let (hand_str, bid_str) = input.split_once(' ').unwrap();
        let hand = Hand::parse(hand_str);
        let bid = bid_str.parse().unwrap();
        HandBid { hand, bid }
    }
}

fn solve1(input: &str) -> u32 {
    let mut hand_bids: Vec<HandBid> = input.lines().map(HandBid::parse).collect();
    hand_bids.sort_by(|a, b| hand_ordering(&a.hand, &b.hand, hand_type, card_value_1).unwrap());
    hand_bids
        .iter()
        .enumerate()
        .map(|(i, hb)| (i as u32 + 1) * hb.bid)
        .sum()
}

fn solve2(input: &str) -> u32 {
    let mut hand_bids: Vec<HandBid> = input.lines().map(HandBid::parse).collect();
    hand_bids.sort_by(|a, b| hand_ordering(&a.hand, &b.hand, hand_type_2, card_value_2).unwrap());
    hand_bids
        .iter()
        .enumerate()
        .map(|(i, hb)| (i as u32 + 1) * hb.bid)
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

    #[test]
    fn card_ordering1() {
        assert_eq!(
            hand_ordering(
                &Hand::parse("33332"),
                &Hand::parse("2AAAA"),
                hand_type,
                card_value_1
            ),
            Some(std::cmp::Ordering::Greater)
        );
        assert_eq!(
            hand_ordering(
                &Hand::parse("77888"),
                &Hand::parse("77788"),
                hand_type,
                card_value_1
            ),
            Some(std::cmp::Ordering::Greater)
        );
        assert_eq!(
            hand_ordering(
                &Hand::parse("22222"),
                &Hand::parse("22344"),
                hand_type,
                card_value_1
            ),
            Some(std::cmp::Ordering::Greater)
        );
    }

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
