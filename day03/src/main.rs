use std::collections::{HashMap, HashSet};

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn surrounding(&self) -> HashSet<Coord> {
        let mut result = HashSet::new();

        let start_row = if self.row == 0 { 0 } else { self.row - 1 };
        let start_col = if self.col == 0 { 0 } else { self.col - 1 };

        for row in start_row..self.row + 2 {
            for col in start_col..self.col + 2 {
                let here = Coord { row, col };
                if &here != self {
                    result.insert(here);
                }
            }
        }
        result
    }
}

#[derive(Clone, Debug)]
struct PartNumber {
    number: i32,
    coords: HashSet<Coord>,
}

impl PartNumber {
    fn surrounding(&self) -> HashSet<Coord> {
        let mut result = HashSet::new();
        for coord in &self.coords {
            result.extend(coord.surrounding());
        }
        for coord in &self.coords {
            result.remove(&coord);
        }
        result
    }
}

#[derive(Debug)]
struct Schematic {
    part_numbers: Vec<PartNumber>,
    symbols: HashMap<Coord, char>,
}

fn parse(input: &str) -> Schematic {
    let mut part_numbers = Vec::new();
    let mut symbols = HashMap::new();
    let mut current_number: Option<PartNumber> = None;

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            let here = Coord { row, col };
            match (char, &current_number) {
                (s, Some(num)) if !s.is_digit(10) => {
                    part_numbers.push(num.clone());
                    current_number = None;

                    if s != '.' {
                        symbols.insert(here, s);
                    }
                }
                (s, None) if s.is_digit(10) => {
                    let mut coords = HashSet::new();
                    coords.insert(here);

                    current_number = Some(PartNumber {
                        number: i32::try_from(s.to_digit(10).unwrap()).unwrap(),
                        coords,
                    });
                }
                (s, Some(num)) if s.is_digit(10) => {
                    let mut coords = num.coords.clone();
                    coords.insert(here);
                    current_number = Some(PartNumber {
                        number: num.number * 10 + i32::try_from(s.to_digit(10).unwrap()).unwrap(),
                        coords,
                    });
                }
                (s, _) if s != '.' => {
                    symbols.insert(here, s);
                }
                _ => {}
            }
        }
        match &current_number {
            Some(num) => {
                part_numbers.push(num.clone());
                current_number = None;
            }
            _ => {}
        }
    }

    Schematic {
        part_numbers,
        symbols,
    }
}

fn solve1(input: &str) -> i32 {
    let schematic = parse(input);
    let symbol_coords = schematic.symbols.keys().map(|c| c.clone()).collect();

    schematic
        .part_numbers
        .into_iter()
        .filter(|pn| !pn.surrounding().is_disjoint(&symbol_coords))
        .map(|pn| pn.number)
        .sum()
}

fn solve2(input: &str) -> i32 {
    let schematic = parse(input);

    let maybe_gear_coords: Vec<Coord> = schematic
        .symbols
        .into_iter()
        .filter_map(|(coord, c)| if c == '*' { Some(coord) } else { None })
        .collect();
    dbg!(&maybe_gear_coords.len());

    let part_numbers_to_surrounding: Vec<(i32, HashSet<Coord>)> = schematic
        .part_numbers
        .iter()
        .map(|pn| (pn.number, pn.surrounding()))
        .collect();

    let gear_ratios = maybe_gear_coords
        .into_iter()
        .filter_map(|maybe_gear_coord| {
            let part_numbers_adjacent: Vec<i32> = part_numbers_to_surrounding
                .iter()
                .filter_map(|(num, surrounding_coords)| {
                    if surrounding_coords.contains(&maybe_gear_coord) {
                        Some(num.to_owned())
                    } else {
                        None
                    }
                })
                .collect();
            if maybe_gear_coord.row == 1 {
                dbg!(&maybe_gear_coord, &part_numbers_adjacent);
            }

            if part_numbers_adjacent.len() == 2 {
                Some(part_numbers_adjacent[0] * part_numbers_adjacent[1])
            } else {
                None
            }
        });

    gear_ratios.sum()
    // 31285222: too low
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
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 4361);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 467835);
    }
}
