use std::{
    cmp::{max, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Debug,
};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumIter)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn step(&self, direction: &Direction, num_rows: usize, num_cols: usize) -> Option<Self> {
        match direction {
            Direction::Up if self.row > 0 => Some(Coord {
                row: self.row - 1,
                col: self.col,
            }),
            Direction::Right if self.col < num_cols - 1 => Some(Coord {
                row: self.row,
                col: self.col + 1,
            }),
            Direction::Down if self.row < num_rows - 1 => Some(Coord {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    coord: Coord,
    direction: Direction,
    direction_step_count: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct StateWithCost {
    state: State,
    cost: u32,
}

impl PartialOrd for StateWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StateWithCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

struct PuzzleInput {
    grid: HashMap<Coord, u32>,
    num_rows: usize,
    num_cols: usize,
}

impl PuzzleInput {
    fn parse(input: &str) -> Self {
        let mut grid = HashMap::new();
        let mut num_rows = 0;
        let mut num_cols = 0;
        for (row, tiles) in input.lines().enumerate() {
            for (col, tile) in tiles.chars().enumerate() {
                grid.insert(Coord { row, col }, tile.to_digit(10).unwrap());
                num_cols = max(col + 1, num_cols);
            }
            num_rows += 1;
        }
        Self {
            grid,
            num_rows,
            num_cols,
        }
    }
}

struct Todo<T> {
    heap: BinaryHeap<Reverse<T>>,
}

impl<T: Ord> Todo<T> {
    fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.heap.push(Reverse(item))
    }

    fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|reverse| reverse.0)
    }
}

fn find_shortest_path(puzzle: PuzzleInput, min_steps: u8, max_steps: u8) -> u32 {
    let start = Coord { row: 0, col: 0 };
    let target = Coord {
        row: puzzle.num_rows - 1,
        col: puzzle.num_cols - 1,
    };

    let mut todo: Todo<StateWithCost> = Todo::new();
    let mut seen: HashSet<State> = HashSet::new();

    for direction in [Direction::Right, Direction::Down] {
        let neighbor_coord = start
            .step(&direction, puzzle.num_rows, puzzle.num_cols)
            .unwrap();

        todo.push(StateWithCost {
            state: State {
                coord: neighbor_coord,
                direction,
                direction_step_count: 1,
            },
            cost: *puzzle.grid.get(&neighbor_coord).unwrap(),
        });
    }

    while let Some(state_with_cost) = todo.pop() {
        if seen.contains(&state_with_cost.state) {
            continue; // must have been here before with better cost
        }
        seen.insert(state_with_cost.state);

        if state_with_cost.state.coord == target
            && state_with_cost.state.direction_step_count >= min_steps
        {
            return state_with_cost.cost;
        }

        for direction in Direction::iter() {
            // println!("checking step direction {:?}", direction);
            let turning_allowed = state_with_cost.state.direction_step_count >= min_steps;
            if let Some(step_coord) =
                state_with_cost
                    .state
                    .coord
                    .step(&direction, puzzle.num_rows, puzzle.num_cols)
            {
                let mut direction_step_count = 1;

                if direction == state_with_cost.state.direction {
                    if state_with_cost.state.direction_step_count == max_steps {
                        continue;
                    }
                    direction_step_count = state_with_cost.state.direction_step_count + 1;
                } else if !turning_allowed
                    || direction == state_with_cost.state.direction.opposite()
                {
                    continue;
                }

                todo.push(StateWithCost {
                    state: State {
                        coord: step_coord,
                        direction,
                        direction_step_count,
                    },
                    cost: state_with_cost.cost + puzzle.grid.get(&step_coord).unwrap(),
                });
            }
        }
    }
    panic!("could not reach the target")
}

fn solve1(input: &str) -> u32 {
    let puzzle = PuzzleInput::parse(input);
    find_shortest_path(puzzle, 1, 3)
}

fn solve2(input: &str) -> u32 {
    let puzzle = PuzzleInput::parse(input);
    find_shortest_path(puzzle, 4, 10)
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
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 102);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 94);
    }

    #[test]
    fn example2_unfortunate() {
        assert_eq!(
            solve2(
                "\
111111111111
999999999991
999999999991
999999999991
999999999991"
            ),
            71
        )
    }
}
