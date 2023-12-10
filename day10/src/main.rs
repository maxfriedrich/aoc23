use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    row: i32,
    col: i32,
}

impl Coord {
    fn up(&self) -> Coord {
        Coord {
            row: self.row - 1,
            col: self.col,
        }
    }

    fn down(&self) -> Coord {
        Coord {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn left(&self) -> Coord {
        Coord {
            row: self.row,
            col: self.col - 1,
        }
    }

    fn right(&self) -> Coord {
        Coord {
            row: self.row,
            col: self.col + 1,
        }
    }

    fn neighbors(&self) -> [Coord; 4] {
        [self.up(), self.down(), self.left(), self.right()]
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Start,
    Ground,
    Pipe { connections: (Coord, Coord) },
}

impl Tile {
    fn parse(input: char, coord: Coord) -> Tile {
        match input {
            '.' => Tile::Ground,
            'S' => Tile::Start,
            '|' => Tile::Pipe {
                connections: (coord.up(), coord.down()),
            },
            '-' => Tile::Pipe {
                connections: (coord.left(), coord.right()),
            },
            'L' => Tile::Pipe {
                connections: (coord.up(), coord.right()),
            },
            'J' => Tile::Pipe {
                connections: (coord.up(), coord.left()),
            },
            '7' => Tile::Pipe {
                connections: (coord.down(), coord.left()),
            },
            'F' => Tile::Pipe {
                connections: (coord.down(), coord.right()),
            },
            _ => panic!("invalid tile: {}", input),
        }
    }

    fn out(&self, in_coord: &Coord) -> Option<Coord> {
        match self {
            Tile::Pipe { connections } => {
                if in_coord == &connections.0 {
                    Some(connections.1)
                } else if in_coord == &connections.1 {
                    Some(connections.0)
                } else {
                    None
                }
            }
            _ => panic!("not implemented for non-pipe tiles"),
        }
    }
}

fn parse_grid(input: &str) -> HashMap<Coord, Tile> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(col, c)| {
                let coord = Coord {
                    row: row as i32,
                    col: col as i32,
                };
                (coord, Tile::parse(c, coord))
            })
        })
        .collect()
}

fn follow_pipes(start: &Coord, target: &Coord, grid: &HashMap<Coord, Tile>) -> Option<Vec<Coord>> {
    // dbg!(&start);
    let mut visited: Vec<Coord> = vec![*target];
    let mut current = *start;

    while &current != target {
        match grid.get(&current) {
            Some(tile) => {
                if let Tile::Pipe { .. } = tile {
                    // dbg!(&visited, &current, &tile);
                    if let Some(new_current) = tile.out(visited.last().unwrap()) {
                        visited.push(current);
                        current = new_current;
                    } else {
                        return None;
                    }
                }
            }
            _ => return None,
        }
    }
    Some(visited)
}

fn find_loop(grid: &HashMap<Coord, Tile>) -> Vec<Coord> {
    let start_coord = grid
        .iter()
        .find(|(_, t)| matches!(t, Tile::Start { .. }))
        .map(|x| *x.0)
        .unwrap();

    for neighbor_coord in start_coord.neighbors() {
        match grid.get(&neighbor_coord) {
            Some(tile) => {
                let maybe_visited = if let Tile::Pipe { .. } = tile {
                    follow_pipes(&neighbor_coord, &start_coord, grid)
                } else {
                    None
                };

                if let Some(visited) = maybe_visited {
                    return visited;
                }
            }
            _ => continue,
        }
    }
    panic!()
}

fn solve1(input: &str) -> usize {
    let grid = parse_grid(input);
    let pipe_loop = find_loop(&grid);
    pipe_loop
        .iter()
        .enumerate()
        .map(|(i, _)| i.min(pipe_loop.len() - i))
        .max()
        .unwrap()
}

fn expand_loop(tiles: &[Coord]) -> Vec<Coord> {
    let mut tiles_to_process = Vec::new();
    tiles_to_process.extend_from_slice(tiles);
    tiles_to_process.push(*tiles.first().unwrap());

    let mut result = Vec::new();

    for tile_pair in tiles_to_process.windows(2) {
        let (a, b) = (tile_pair.first().unwrap(), tile_pair.last().unwrap());
        let row_diff = b.row - a.row;
        let col_diff = b.col - a.col;

        result.push(Coord {
            row: a.row * 2,
            col: a.col * 2,
        });
        result.push(Coord {
            row: a.row * 2 + row_diff,
            col: a.col * 2 + col_diff,
        })
    }
    result
}

fn solve2(input: &str) -> usize {
    let grid = parse_grid(input);
    let pipe_loop = find_loop(&grid);

    // Idea: make the loop 2x larger by inserting virtual tiles so || becomes |.|
    // Then repeatedly check all non-loop tiles for reachability. A tile is reachable if:
    // * a neighbor is reachable
    // * it's on the grid border (= a neighbor is outside the grid)

    let expanded_loop: HashSet<Coord> = expand_loop(&pipe_loop).into_iter().collect();

    let max_row = expanded_loop.iter().map(|c| c.row).max().unwrap();
    let max_col = expanded_loop.iter().map(|c| c.col).max().unwrap();

    let mut non_loop: HashSet<Coord> = HashSet::new();
    for row in 0..max_row + 1 {
        for col in 0..max_col + 1 {
            let c = Coord { row, col };
            if !expanded_loop.contains(&c) {
                non_loop.insert(c);
            }
        }
    }
    // dbg!(&non_loop.len());

    let mut reachable: HashSet<Coord> = HashSet::new();
    let mut done = false;

    while !done {
        // dbg!(&reachable.len());
        done = true;
        for coord in non_loop.iter() {
            if reachable.contains(coord) {
                continue;
            }
            // println!("checking {:?}", &coord);
            let reachable_neighbor = coord.neighbors().iter().any(|c| reachable.contains(c));
            let non_grid_neighbor = coord
                .neighbors()
                .iter()
                .any(|c| c.row < 0 || c.col < 0 || c.row > max_row || c.col > max_col);
            if reachable_neighbor || non_grid_neighbor {
                reachable.insert(*coord);
                done = false;
            }
        }
    }
    let unreachable = non_loop.iter().filter(|k| !reachable.contains(k));
    let unreachable_original: Vec<Coord> = unreachable
        .filter(|c| c.row % 2 == 0 && c.col % 2 == 0)
        .map(|c| Coord {
            row: c.row / 2,
            col: c.col / 2,
        })
        .collect();
    unreachable_original.len()
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", solve1(input));
    println!("{}", solve2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_simple() {
        let input = "\
.....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(solve1(input), 4);
    }

    #[test]
    fn example1_simple_with_noise() {
        let input = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(solve1(input), 4);
    }

    #[test]
    fn example1_complex_with_noise() {
        let input = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(solve1(input), 8);
    }

    #[test]
    fn example2_large() {
        let input = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(solve2(input), 8)
    }

    #[test]
    fn example2_small() {
        let input = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(solve2(input), 4)
    }

    #[test]
    fn example2_small_no_gap() {
        let input = "\
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        assert_eq!(solve2(input), 4)
    }

    #[test]
    fn example2_large_with_noise() {
        let input = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(solve2(input), 8)
    }

    #[test]
    fn example2_large_with_more_noise() {
        let input = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(solve2(input), 10)
    }
}
