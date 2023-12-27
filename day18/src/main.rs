#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn step(&self, direction: &Direction, size: usize) -> Self {
        let step_size: i64 = size.try_into().unwrap();
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y + step_size,
            },
            Direction::Right => Self {
                x: self.x + step_size,
                y: self.y,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y - step_size,
            },
            Direction::Left => Self {
                x: self.x - step_size,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn parse(input: char) -> Self {
        match input {
            'U' => Self::Up,
            'R' => Self::Right,
            'D' => Self::Down,
            'L' => Self::Left,
            _ => panic!("invalid direction: {input}"),
        }
    }

    fn parse_hex(input: char) -> Self {
        match input {
            '0' => Self::Right,
            '1' => Self::Down,
            '2' => Self::Left,
            '3' => Self::Up,
            _ => panic!("invalid direction: {input}"),
        }
    }
}

fn parse1(input: &str) -> Vec<(Direction, usize)> {
    input
        .lines()
        .map(|line| {
            let (direction_part, rest) = line.split_once(' ').unwrap();
            let (num_steps_part, _color_part) = rest.split_once(' ').unwrap();

            let direction = Direction::parse(direction_part.chars().next().unwrap());
            let num_steps = num_steps_part.parse().unwrap();

            (direction, num_steps)
        })
        .collect()
}

fn parse2(input: &str) -> Vec<(Direction, usize)> {
    input
        .lines()
        .map(|line| {
            let instruction_part = line.split(' ').last().unwrap();
            let num_steps = usize::from_str_radix(&instruction_part[2..7], 16).unwrap();
            let direction = Direction::parse_hex(instruction_part.chars().nth(7).unwrap());

            (direction, num_steps)
        })
        .collect()
}

fn dig(instructions: &[(Direction, usize)]) -> Vec<Coord> {
    let mut dug = Vec::new();
    let mut current = Coord { x: 0, y: 0 };
    dug.push(current);

    for (direction, num_steps) in instructions {
        for _ in 0..*num_steps {
            current = current.step(direction, 1);
            dug.push(current);
        }
    }
    dug
}

fn dig2(instructions: &[(Direction, usize)]) -> (Vec<Coord>, u64) {
    let mut dug = Vec::new();
    let mut border_length = 0;
    let mut current = Coord { x: 0, y: 0 };
    dug.push(current);

    for (direction, num_steps) in instructions {
        border_length += *num_steps as u64;
        current = current.step(direction, *num_steps);
        dug.push(current);
    }
    (dug, border_length)
}

// got some help here: https://advent-of-code.xavd.id/writeups/2023/day/18/
fn num_points_in_shape(coords: &[Coord], border_length: Option<u64>) -> u64 {
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let area: f64 = coords
        .windows(2)
        .map(|window| {
            let a = window[0];
            let b = window[1];
            a.y * b.x - b.y * a.x
        })
        .sum::<i64>() as f64
        / 2.0;

    let border_length = border_length.unwrap_or(coords.len() as u64) as f64;

    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    (area.abs() - 0.5 * border_length + 1.0 + border_length) as u64
}

fn solve1(input: &str) -> u64 {
    let instructions = parse1(input);
    let dug = dig(&instructions);
    num_points_in_shape(&dug, None)
}

fn solve2(input: &str) -> u64 {
    let instructions = parse2(input);
    let (dug, border_length) = dig2(&instructions);
    num_points_in_shape(&dug, Some(border_length))
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
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 62);
    }

    #[test]
    fn example2() {
        (solve2(EXAMPLE), 952408144115_u64);
    }
}
