struct Card {
    #[allow(dead_code)]
    id: i32,
    mine: Vec<i32>,
    winning: Vec<i32>,
}

impl Card {
    fn parse(input: &str) -> Card {
        let input = input.strip_prefix("Card").unwrap().trim_start();
        let (id_str, remaining) = input.split_once(':').unwrap();
        let id: i32 = id_str.parse().unwrap();

        let (mine_str, winning_str) = remaining.split_once('|').unwrap();
        let mine: Vec<i32> = mine_str
            .split_whitespace()
            .map(|num_str| num_str.trim().parse().unwrap())
            .collect();
        let winning: Vec<i32> = winning_str
            .split_whitespace()
            .map(|num_str| num_str.trim().parse().unwrap())
            .collect();

        Card { id, mine, winning }
    }

    fn num_matching(&self) -> u32 {
        self.mine
            .iter()
            .filter(|m| self.winning.contains(m))
            .count()
            .try_into()
            .unwrap()
    }

    fn value(&self) -> i32 {
        match self.num_matching() {
            0 => 0,
            n => 2_i32.pow(n - 1),
        }
    }
}

fn solve1(input: &str) -> i32 {
    let cards = input.lines().map(Card::parse);
    cards.map(|card| card.value()).sum()
}

fn solve2(input: &str) -> u32 {
    let cards: Vec<Card> = input.lines().map(Card::parse).collect();
    let cards_num_matching: Vec<u32> = cards.iter().map(|c| c.num_matching()).collect();
    let mut result: Vec<u32> = cards.iter().map(|_| 1).collect();

    for current_card in 0..(cards.len()) {
        let num_card_instances = result[current_card];
        let num_matching: usize = cards_num_matching[current_card].try_into().unwrap();

        for won_card in (current_card + 1)..(current_card + num_matching + 1) {
            result[won_card] += num_card_instances;
        }
    }

    result.iter().sum()
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
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 13);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 30);
    }
}
