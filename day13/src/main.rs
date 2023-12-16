use std::cmp::min;

fn parse_tile(input: char) -> u8 {
    match input {
        '.' => 0,
        '#' => 1,
        _ => panic!(),
    }
}

fn bits_to_u32(bits: &[u8]) -> u32 {
    let mut result = 0;
    for &bit in bits {
        result = (result << 1) | u32::from(bit);
    }
    result
}

#[derive(Debug)]
struct Pattern {
    rows: Vec<u32>,
    cols: Vec<u32>,
    row_size: usize,
    col_size: usize,
    #[allow(dead_code)]
    original: String,
}

impl Pattern {
    fn parse(input: &str) -> Self {
        let grid_by_rows: Vec<Vec<u8>> = input
            .lines()
            .map(|line| line.chars().map(parse_tile).collect())
            .collect();

        let mut grid_by_cols = vec![Vec::new(); grid_by_rows.first().unwrap().len()];
        for row in &grid_by_rows {
            for (col_num, tile) in row.iter().enumerate() {
                grid_by_cols[col_num].push(*tile);
            }
        }

        Self {
            rows: grid_by_rows.iter().map(|bits| bits_to_u32(bits)).collect(),
            cols: grid_by_cols.iter().map(|bits| bits_to_u32(bits)).collect(),
            row_size: grid_by_rows.first().unwrap().len(),
            col_size: grid_by_cols.first().unwrap().len(),
            original: input.to_string(),
        }
    }
}

fn find_mirrors(ids: &[u32]) -> Vec<usize> {
    let mut result = Vec::new();
    for mirror_position in 1..ids.len() {
        let num_after = ids.len() - mirror_position;
        let mirrored_size = min(num_after, mirror_position);

        let m1 = &ids[mirror_position - mirrored_size..mirror_position];
        let m2: Vec<u32> = ids[mirror_position..mirror_position + mirrored_size]
            .iter()
            .rev()
            .copied()
            .collect();

        if m1 == m2 {
            result.push(mirror_position);
        }
    }
    result
}

fn find_mirror(ids: &[u32]) -> Option<usize> {
    find_mirrors(ids).first().copied()
}

fn find_mirror_smudged(ids: &[u32], size: usize) -> Option<usize> {
    let original_mirror = find_mirror(ids);

    for (i, a) in ids.iter().enumerate() {
        for b in ids {
            if a == b {
                continue;
            }
            for bit in 0..size {
                // check if setting or unsetting this bit transforms a into b
                if (a | (1 << bit) == *b) || (a & !(1 << bit) == *b) {
                    let mut new_ids = ids.to_owned();
                    new_ids[i] = *b;
                    let new_mirror = find_mirrors(&new_ids)
                        .into_iter()
                        .find(|&m| original_mirror != Some(m));
                    if new_mirror.is_some() {
                        return new_mirror;
                    }
                }
            }
        }
    }
    None
}

fn parse_input(input: &str) -> Vec<Pattern> {
    input.split("\n\n").map(Pattern::parse).collect()
}

fn solve1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|p| {
            find_mirror(&p.cols)
                .or(find_mirror(&p.rows).map(|m| m * 100))
                .unwrap()
        })
        .sum()
}

fn solve2(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|p| {
            find_mirror_smudged(&p.cols, p.col_size)
                .or(find_mirror_smudged(&p.rows, p.row_size).map(|m| m * 100))
                .unwrap()
        })
        .sum()
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
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 405);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 400);
    }
}
