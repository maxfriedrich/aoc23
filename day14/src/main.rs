use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Tile {
    Round,
    Cube,
    Empty,
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Tile {
    fn parse(input: char) -> Self {
        match input {
            'O' => Self::Round,
            '#' => Self::Cube,
            '.' => Self::Empty,
            _ => panic!(),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Self::Round => "O",
            Self::Cube => "#",
            Self::Empty => ".",
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    row: usize,
    col: usize,
}

struct Grid {
    tiles: HashMap<Coord, Tile>,
    num_rows: usize,
    num_cols: usize,
    tilt_indices: HashMap<Direction, (Vec<usize>, Vec<usize>)>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut tiles = HashMap::new();
        let mut num_rows = 0;
        let mut num_cols = 0;
        for (row, line) in input.lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                if row == 0 {
                    num_cols += 1;
                }
                tiles.insert(Coord { row, col }, Tile::parse(char));
            }
            num_rows += 1;
        }
        let tilt_indices = vec![
            (
                Direction::North,
                ((0..num_cols).collect(), (1..num_rows).collect()),
            ),
            (
                Direction::East,
                ((0..num_rows).collect(), (0..num_cols - 1).rev().collect()),
            ),
            (
                Direction::South,
                ((0..num_cols).collect(), (0..num_rows - 1).rev().collect()),
            ),
            (
                Direction::West,
                ((0..num_rows).collect(), (1..num_cols).collect()),
            ),
        ]
        .into_iter()
        .collect();

        Grid {
            tiles,
            num_cols,
            num_rows,
            tilt_indices,
        }
    }

    fn to_string(&self) -> String {
        (0..self.num_rows)
            .map(|row| {
                (0..self.num_cols)
                    .map(|col| self.tiles.get(&Coord { row, col }).unwrap().to_str())
                    .collect::<Vec<&str>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn tilt(&mut self, direction: &Direction) {
        let (outer, inner) = self.tilt_indices.get(direction).unwrap();
        for o in outer {
            // the dimension where tiles don't interact with each other
            let mut done = false;
            while !done {
                done = true;
                for i in inner {
                    // the dimension where we may need to swap
                    let (t1_coord, t2_coord) = match direction {
                        Direction::North => (
                            Coord { row: *i, col: *o },
                            Coord {
                                row: *i - 1,
                                col: *o,
                            },
                        ),
                        Direction::East => (
                            Coord { row: *o, col: *i },
                            Coord {
                                row: *o,
                                col: *i + 1,
                            },
                        ),
                        Direction::South => (
                            Coord { row: *i, col: *o },
                            Coord {
                                row: *i + 1,
                                col: *o,
                            },
                        ),
                        Direction::West => (
                            Coord { row: *o, col: *i },
                            Coord {
                                row: *o,
                                col: *i - 1,
                            },
                        ),
                    };

                    let t1 = self.tiles.get(&t1_coord).unwrap();
                    let t2 = self.tiles.get(&t2_coord).unwrap();

                    if t1 == &Tile::Round && t2 == &Tile::Empty {
                        self.tiles.insert(t1_coord, Tile::Empty);
                        self.tiles.insert(t2_coord, Tile::Round);
                        done = false;
                    }
                }
            }
        }
    }

    fn find_round_tiles(&self) -> HashSet<Coord> {
        self.tiles
            .iter()
            .filter(|(_, &tile)| tile == Tile::Round)
            .map(|(coord, _)| *coord)
            .collect()
    }

    fn compute_load(&self) -> usize {
        self.find_round_tiles()
            .iter()
            .map(|coord| self.num_rows - coord.row)
            .sum()
    }
}

fn solve1(input: &str) -> usize {
    let mut grid = Grid::parse(input);
    grid.tilt(&Direction::North);
    grid.compute_load()
}

const NUM_TILT_CYCLES: usize = 1000000000;

fn solve2(input: &str) -> usize {
    let mut grid = Grid::parse(input);

    // find the cycle and extrapolate
    let mut seen: HashMap<Vec<Coord>, usize> = HashMap::new();
    let mut loads: Vec<usize> = Vec::new();
    loads.push(grid.compute_load());

    let tilt_until_billion = (1..NUM_TILT_CYCLES).map(|i| {
        grid.tilt(&Direction::North);
        grid.tilt(&Direction::West);
        grid.tilt(&Direction::South);
        grid.tilt(&Direction::East);

        let round_coords = grid.find_round_tiles();
        let load = grid.compute_load();
        (i, (round_coords, load))
    });

    for (current_step, (round_coords, load)) in tilt_until_billion {
        loads.push(load);
        // println!("{load}");
        let mut sorted_coords: Vec<Coord> = round_coords.into_iter().collect();
        sorted_coords.sort();

        if let Some(previous_step) = seen.get(&sorted_coords) {
            let steps_before_cycle = previous_step;
            let cycle_length = current_step - previous_step;

            return loads
                [steps_before_cycle + (NUM_TILT_CYCLES - steps_before_cycle) % cycle_length];
        } else {
            seen.insert(sorted_coords, current_step);
        }
    }
    panic!()
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", solve1(input));
    print!("{} ", solve2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn example1() {
        let mut grid = Grid::parse(EXAMPLE);

        grid.tilt(&Direction::North);
        assert_eq!(
            grid.to_string(),
            "\
OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
        );

        assert_eq!(grid.compute_load(), 136);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 64);
    }
}
