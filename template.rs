fn solve1(input: &str) -> u32 {
    todo!()
}

fn solve2(input: &str) -> u32 {
    todo!()
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
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

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
