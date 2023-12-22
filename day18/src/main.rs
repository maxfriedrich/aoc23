use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn step(&self, direction: &Direction, size: usize) -> Self {
        let step_size: i32 = size.try_into().unwrap();
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
            _ => panic!("invalid direction"),
        }
    }
}

fn parse(input: &str) -> Vec<(Direction, usize)> {
    input
        .lines()
        .map(|line| {
            let (direction_part, rest) = line.split_once(' ').unwrap();
            let (num_steps_part, _color_part) = rest.split_once(' ').unwrap();

            let direction = Direction::parse(direction_part.chars().nth(0).unwrap());
            let num_steps = num_steps_part.parse().unwrap();

            (direction, num_steps)
        })
        .collect()
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Trench {
    start: Coord,
    end: Coord,
}

fn solve1(input: &str) -> usize {
    let instructions = parse(input);
    let mut dug: HashMap<i32, Trench> = HashMap::new();
    let mut inner: HashSet<Coord> = HashSet::new();
    let mut current = Coord { x: 0, y: 0 };

    for (direction, num_steps) in instructions {
        match direction {
            Direction::Left | Direction::Right => {
                let end = current.step(&direction, num_steps);
                dug.insert(
                    current.y,
                    Trench {
                        start: current,
                        end: end,
                    },
                );
                current = end;
            }
            Direction::Up | Direction::Down => {
                for _ in 0..num_steps {
                    dug.insert(
                        current.y,
                        Trench {
                            start: current,
                            end: current,
                        },
                    );
                    current = current.step(&direction, 1);
                }
            }
        }
    }

    let dug_by_y: HashMap<i32, Vec<i32>> =
        dug.iter()
            .map(|trench| (coord.y, coord.x))
            .fold(HashMap::new(), |mut acc, (y, x)| {
                acc.entry(y).or_insert(Vec::new()).push(x);
                acc
            });
    // combine segments of dug out

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for coord in &dug {
        if coord.x < min_x {
            min_x = coord.x;
        }
        if coord.x > max_x {
            max_x = coord.x;
        }
        if coord.y < min_y {
            min_y = coord.y;
        }
        if coord.y > max_y {
            max_y = coord.y;
        }
    }

    let mut vis: Vec<Vec<char>> = Vec::new();

    for y in min_y..max_y + 1 {
        let mut vis_line = Vec::new();
        for x in min_x..max_x + 1 {
            if dug.contains(&Coord { x, y }) {
                vis_line.push('#');
            } else if dug_by_y
                .get(&y)
                .unwrap()
                .iter()
                .filter(|&dug_x| dug_x < &x)
                .count()
                % 2
                == 1
            {
                inner.insert(Coord { x, y });
                vis_line.push('#');
            } else {
                vis_line.push('.');
            }
        }
        vis.push(vis_line);
    }

    let vis_str = vis
        .iter()
        .map(|chars| {
            chars
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", vis_str);
    // dbg!(&dug);
    // dbg!(&inner);

    dug.union(&inner).count()
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

    // #[test]
    fn example2() {
        (solve2(EXAMPLE), 281);
    }
}
