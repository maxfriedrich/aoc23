fn first_and_last_digit(input: &str) -> u32 {
    let digits_chars = input
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    digits_chars.first().unwrap() * 10 + digits_chars.last().unwrap()
}

const DIGIT_NAMES: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn first_and_last_digit_including_names(input: &str) -> u32 {
    let mut first_ind = usize::MAX;
    let mut first_val = 0;
    let mut last_ind = usize::MIN;
    let mut last_val = 0;

    let mut digits: Vec<(String, u32)> = DIGIT_NAMES
        .iter()
        .map(|(_, n)| (n.to_string(), n.clone()))
        .collect();
    let digit_names: Vec<(String, u32)> = DIGIT_NAMES
        .iter()
        .map(|(s, n)| (s.to_string(), n.clone()))
        .collect();
    digits.extend(digit_names);

    for (digit_str, digit) in digits {
        let digit_first_ind = input.find(&digit_str);
        if digit_first_ind.is_some_and(|i| i < first_ind) {
            first_ind = digit_first_ind.unwrap();
            first_val = digit;
        }
        let digit_last_ind = input.rfind(&digit_str);
        if digit_last_ind.is_some_and(|i| i >= last_ind) {
            last_ind = digit_last_ind.unwrap();
            last_val = digit;
        }
    }

    first_val * 10 + last_val
}

fn solve1(input: &str) -> u32 {
    input.lines().map(first_and_last_digit).sum()
}

fn solve2(input: &str) -> u32 {
    input
        .lines()
        .map(first_and_last_digit_including_names)
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

    #[test]
    fn example1() {
        let input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(solve1(input), 142);
    }

    #[test]
    fn example2() {
        let input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(solve2(input), 281);
    }
}
