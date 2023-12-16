use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    MirrorBottomLeftTopRight,
    MirrorTopLeftBottomRight,
    SplitterHorizontal,
    SplitterVertical,
}

impl Tile {
    fn parse(input: char) -> Self {
        match input {
            '.' => Self::Empty,
            '/' => Self::MirrorBottomLeftTopRight,
            '\\' => Self::MirrorTopLeftBottomRight,
            '-' => Self::SplitterHorizontal,
            '|' => Self::SplitterVertical,
            _ => panic!("invalid tile"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn step(&self, direction: &Direction) -> Option<Self> {
        match direction {
            Direction::Up if self.row > 0 => Some(Coord {
                row: self.row - 1,
                col: self.col,
            }),
            Direction::Right => Some(Coord {
                row: self.row,
                col: self.col + 1,
            }),
            Direction::Down => Some(Coord {
                row: self.row + 1,
                col: self.col,
            }),
            Direction::Left if self.col > 0 => Some(Coord {
                row: self.row,
                col: self.col - 1,
            }),
            _ => None,
        }
    }
}

fn parse_grid(input: &str) -> HashMap<Coord, Tile> {
    let mut num_rows = 0;
    let mut num_cols = 0;
    let mut result = HashMap::new();
    for (row, tiles) in input.lines().enumerate() {
        // dbg!(tiles);
        for (col, tile) in tiles.chars().enumerate() {
            // dbg!(row, col);
            result.insert(Coord { row, col }, Tile::parse(tile));
            num_cols = max(col + 1, num_cols);
        }
        num_rows += 1;
    }

    assert_eq!(result.len(), num_rows * num_cols);
    for row in 0..num_rows {
        for col in 0..num_cols {
            assert!(result.contains_key(&Coord { row, col }));
        }
    }

    result
}

fn num_energized_tiles(grid: &HashMap<Coord, Tile>, init: (Coord, Direction)) -> usize {
    let mut energized: HashSet<Coord> = HashSet::new();
    let mut todo = vec![init];
    let mut visited: HashSet<(Coord, Direction)> = HashSet::new();

    while let Some((initial_coord, initial_direction)) = todo.pop() {
        let mut current_coord = initial_coord;
        let mut current_direction = initial_direction;

        while let Some(current_tile) = grid.get(&current_coord) {
            if !(visited.insert((current_coord, current_direction))) {
                break;
            };
            energized.insert(current_coord);

            current_direction = match current_tile {
                Tile::Empty => current_direction,
                Tile::MirrorBottomLeftTopRight => match current_direction {
                    // '/' mirror
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                },
                Tile::MirrorTopLeftBottomRight => match current_direction {
                    // '\' mirror
                    Direction::Up => Direction::Left,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                },
                Tile::SplitterHorizontal => match current_direction {
                    Direction::Left | Direction::Right => current_direction,
                    Direction::Down | Direction::Up => {
                        todo.push((current_coord, Direction::Right));
                        Direction::Left
                    }
                },
                Tile::SplitterVertical => match current_direction {
                    Direction::Down | Direction::Up => current_direction,
                    Direction::Left | Direction::Right => {
                        todo.push((current_coord, Direction::Down));
                        Direction::Up
                    }
                },
            };
            if let Some(next_coord) = current_coord.step(&current_direction) {
                current_coord = next_coord;
            } else {
                // stepping outside of grid
                break;
            }
        }
    }

    // print_energized(&energized, &find_max_row_col(grid.keys()));
    energized.len()
}

fn find_max_row_col<'a>(coords: impl Iterator<Item = &'a Coord>) -> (usize, usize) {
    coords.fold((0, 0), |(max_row, max_col), c| {
        (max(max_row, c.row), max(max_col, c.col))
    })
}

#[allow(dead_code)]
fn print_energized(energized: &HashSet<Coord>, max_row_col: &(usize, usize)) {
    let (max_row, max_col) = max_row_col;
    for row in 0..(max_row + 1) {
        for col in 0..(max_col) + 1 {
            print!(
                "{}",
                if energized.contains(&Coord { row, col }) {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
}

fn solve1(input: &str) -> usize {
    let grid = parse_grid(input);
    num_energized_tiles(&grid, (Coord { row: 0, col: 0 }, Direction::Right))
}

fn solve2(input: &str) -> usize {
    let grid = parse_grid(input);
    let (max_row, max_col) = find_max_row_col(grid.keys());

    let mut best = 0;

    for row in 0..(max_row + 1) {
        let from_left = num_energized_tiles(&grid, (Coord { row, col: 0 }, Direction::Right));
        let from_right = num_energized_tiles(&grid, (Coord { row, col: max_col }, Direction::Left));
        best = max(best, max(from_left, from_right));
    }

    for col in 0..(max_col + 1) {
        let from_top = num_energized_tiles(&grid, (Coord { row: 0, col }, Direction::Down));
        let from_bottom = num_energized_tiles(&grid, (Coord { row: max_row, col }, Direction::Up));
        best = max(best, max(from_top, from_bottom));
    }

    best
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", solve1(input));
    println!("{}", solve2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 46);
    }

    #[test]
    fn examples_from_reddit() {
        assert_eq!(
            solve1(
                r#"
|....-
......
......
......
-....|"#
                    .trim()
            ),
            18
        );

        assert_eq!(
            solve1(
                r#"
......|...\..\...
..../........|...
....\.-.../......
......|....../...
................."#
                    .trim()
            ),
            41
        );

        assert_eq!(
            solve1(
                r#"
\........-.........\................................|..................-.............\.
........|....\.../...-...............\.........\...........-......-.......\...../......
.................................../.........................|....|.....\............./
.........\................|..../.........................................-......|......
.|............-....|.....-.....|...............-.............-.........................
...|.....-.|........\....|....................|....|......-.../..............|.....\...
..../.-......|................/.....\......................................./.........-
..-...............\............./.......\......\....-..........\.|.....|.........-.....
...|.................\./.....\.......-.........-................\-.....................
..................................-.../.........../...|...........................-....
..../.....................|..\.|............./....|......................\.........../.
......-/.............|-.../.....|...........././..\...........................\.......\
-.........................|.....\...................|.\.......|.....//..........|......
.......-........../.......\.........|..../........-.|....../....../....-......../..-..-
..-/.....-..//......./.....|.............-....|............/.........\....|........|...
.....-........|.-.|........-.....................-/...\...............................-"#
                    .trim()
            ),
            298
        );

        assert_eq!(
            solve1(
                r#"
\...\.............
.............|/...
....\......-.....|
|.....-....\.|....
............../.|.
.-.-...|....-.-...
..........\.....|.
...../............
......\......\....
.....|./..........
...../...../......
..\...............
....|.........-.|.
.........-........
.............|....
................./"#
                    .trim()
            ),
            16
        )
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 51);
    }
}
