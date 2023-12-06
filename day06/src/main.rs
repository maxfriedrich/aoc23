use std::iter::zip;

fn parse_number_list(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn num_ways_to_win_bruteforce(&self) -> usize {
        (0..self.time + 1)
            .filter(|hold_time| {
                let remaining_time = self.time - hold_time;
                let travel_speed = hold_time;
                let distance_traveled = remaining_time * travel_speed;
                distance_traveled > self.distance
            })
            .count()
    }
}

fn parse_1(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times: Vec<u64> = lines
        .next()
        .and_then(|line| line.strip_prefix("Time: "))
        .map(parse_number_list)
        .unwrap();
    let distances: Vec<u64> = lines
        .next()
        .and_then(|line| line.strip_prefix("Distance: "))
        .map(parse_number_list)
        .unwrap();

    zip(times, distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn parse_2(input: &str) -> Race {
    let mut lines = input.lines();
    let time: u64 = lines
        .next()
        .and_then(|line| line.strip_prefix("Time: "))
        .map(|line| line.replace(' ', ""))
        .map(|line| line.parse().unwrap())
        .unwrap();
    let distance: u64 = lines
        .next()
        .and_then(|line| line.strip_prefix("Distance: "))
        .map(|line| line.replace(' ', ""))
        .map(|line| line.parse().unwrap())
        .unwrap();
    Race { time, distance }
}

fn solve1(input: &str) -> usize {
    let races = parse_1(input);
    races
        .iter()
        .map(|r| r.num_ways_to_win_bruteforce())
        .product()
}

fn solve2(input: &str) -> usize {
    let race = parse_2(input);
    race.num_ways_to_win_bruteforce()
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
Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 288);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 71503);
    }
}
