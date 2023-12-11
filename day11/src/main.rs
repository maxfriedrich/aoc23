use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn distance(&self, other: &Coord) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

struct Universe {
    galaxies: Vec<Coord>,
    empty_rows: HashSet<usize>,
    empty_cols: HashSet<usize>,
}

impl Universe {
    const GALAXY: &'static char = &'#';
    const EMPTY: &'static char = &'.';

    fn parse(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut galaxies: Vec<Coord> = Vec::new();
        for (row, line) in grid.iter().enumerate() {
            for (col, char) in line.iter().enumerate() {
                if char == Self::GALAXY {
                    galaxies.push(Coord { row, col });
                }
            }
        }

        // find empty rows by checking if all chars are EMPTY
        let mut empty_rows: HashSet<usize> = HashSet::new();
        for (row, line) in grid.iter().enumerate() {
            if line.iter().all(|c| c == Self::EMPTY) {
                empty_rows.insert(row);
            }
        }

        // find empty cols by removing cols from a set if they have a non-EMPTY char
        let mut empty_cols: HashSet<usize> =
            HashSet::from_iter(grid.first().unwrap().iter().enumerate().map(|(i, _)| i));
        for (_, line) in grid.iter().enumerate() {
            for (col, char) in line.iter().enumerate() {
                if char != Self::EMPTY {
                    empty_cols.remove(&col);
                }
            }
        }

        Self {
            galaxies,
            empty_rows,
            empty_cols,
        }
    }

    fn expand(&self, empty_size: usize) -> Vec<Coord> {
        let mut new_coords: HashMap<Coord, Coord> =
            HashMap::from_iter(self.galaxies.iter().map(|c| (*c, *c)));

        for raw_coord in &self.galaxies {
            for row in &self.empty_rows {
                if raw_coord.row > *row {
                    new_coords.get_mut(raw_coord).unwrap().row += empty_size - 1;
                }
            }
            for col in &self.empty_cols {
                if raw_coord.col > *col {
                    new_coords.get_mut(raw_coord).unwrap().col += empty_size - 1;
                }
            }
        }

        new_coords.values().copied().collect()
    }
}

fn pairwise_distances_sum(coords: &[Coord]) -> usize {
    coords
        .iter()
        .combinations(2)
        .map(|items| items.first().unwrap().distance(items.last().unwrap()))
        .sum()
}

fn solve1(input: &str) -> usize {
    let universe = Universe::parse(input);
    pairwise_distances_sum(&universe.expand(2))
}

fn solve2(input: &str) -> usize {
    let universe = Universe::parse(input);
    pairwise_distances_sum(&universe.expand(1_000_000))
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
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 374);
    }

    #[test]
    fn example2() {
        let universe = Universe::parse(EXAMPLE);
        assert_eq!(pairwise_distances_sum(&universe.expand(10)), 1030);
        assert_eq!(pairwise_distances_sum(&universe.expand(100)), 8410);
    }
}
