fn parse_number_list(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(parse_number_list).collect()
}

fn differences(history: &[i32]) -> Vec<i32> {
    history
        .windows(2)
        .map(|nums| nums.last().unwrap() - nums.first().unwrap())
        .collect()
}

fn extrapolate(history: &[i32]) -> i32 {
    if history.iter().all(|d| d == &0) {
        0
    } else {
        let diffs = differences(history);
        history.last().unwrap() + extrapolate(&diffs)
    }
}

fn extrapolate_backwards(history: &mut [i32]) -> i32 {
    history.reverse();
    extrapolate(history)
}

fn solve1(input: &str) -> i32 {
    let histories = parse_input(input);
    histories.iter().map(|x| extrapolate(x)).sum()
}

fn solve2(input: &str) -> i32 {
    let mut histories = parse_input(input);
    histories.iter_mut().map(|x| extrapolate_backwards(x)).sum()
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
    0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";

    #[test]
    fn full_example1() {
        assert_eq!(solve1(EXAMPLE), 114);
    }

    #[test]
    fn example1_step_by_step() {
        assert_eq!(extrapolate(&vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(extrapolate(&vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(extrapolate(&vec![10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 2)
    }
}
