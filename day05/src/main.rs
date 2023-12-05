use std::{
    cmp::{max, min},
    collections::HashSet,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Range {
    start: u64,
    length: u64,
}

impl Range {
    fn last(&self) -> u64 {
        self.start + self.length - 1
    }

    fn overlap(&self, other: &Range) -> Option<Range> {
        if self.start > other.last() || other.start > self.last() {
            None
        } else {
            let start = max(self.start, other.start);
            let last = min(self.last(), other.last());
            let length = last + 1 - start;
            Some(Range { start, length })
        }
    }
}

#[derive(Debug)]
struct RangeMapping {
    destination: Range,
    source: Range,
}

#[derive(Debug)]
struct Map {
    #[allow(dead_code)]
    name: String,
    ranges: Vec<RangeMapping>,
}

impl Map {
    fn parse(input: &str) -> Map {
        let mut lines = input.lines();
        let name: String = lines
            .next()
            .unwrap()
            .strip_suffix(" map:")
            .unwrap()
            .to_string();

        let ranges = lines
            .map(parse_number_list)
            .map(|nums| RangeMapping {
                destination: Range {
                    start: nums[0],
                    length: nums[2],
                },
                source: Range {
                    start: nums[1],
                    length: nums[2],
                },
            })
            .collect();

        Map { name, ranges }
    }

    fn get(&self, input_num: u64) -> u64 {
        for range_mapping in &self.ranges {
            let (source, dest) = (&range_mapping.source, &range_mapping.destination);
            if input_num >= source.start && input_num < source.start + source.length {
                return dest.start + (input_num - source.start);
            }
        }
        return input_num;
    }

    fn get_range(&self, input_range: Range) -> Vec<Range> {
        // println!("Current mapping: {}", self.name);
        let mut result = Vec::new();
        let mut remaining = HashSet::new();
        remaining.insert(input_range);

        for range_mapping in &self.ranges {
            // println!("Processing range mapping: {:?}", &range_mapping);
            for current_range in remaining.clone() {
                // println!("  - Processing remaining range: {:?}", &current_range);
                // println!("  - range mapping source: {:?}", &range_mapping.source);
                if let Some(overlap) = range_mapping.source.overlap(&current_range) {
                    remaining.remove(&current_range);

                    // println!("  - Overlap found: {:?} - {}", &overlap, &overlap.last());
                    if overlap.start > current_range.start {
                        remaining.insert(Range {
                            start: current_range.start,
                            length: overlap.start - current_range.start,
                        });
                    }

                    if overlap.last() < current_range.last() {
                        remaining.insert(Range {
                            start: overlap.last() + 1,
                            length: current_range.last() - overlap.last(),
                        });
                    }

                    let destination_range = Range {
                        start: range_mapping.destination.start
                            + (overlap.start - range_mapping.source.start),
                        length: overlap.length,
                    };
                    result.push(destination_range);
                }
            }
            // println!("- remaining ranges: {:?}", &remaining);
            // println!("- result ranges: {:?}", &result);
        }
        for remaining_range in remaining {
            result.push(remaining_range);
        }

        result
    }
}

#[derive(Debug)]
struct PuzzleInput {
    seeds_1: Vec<u64>,
    seeds_2: Vec<Range>,
    maps: Vec<Map>,
}

fn parse_number_list(input: &str) -> Vec<u64> {
    input
        .split(' ')
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}

impl PuzzleInput {
    fn parse(input: &str) -> PuzzleInput {
        let mut parts = input.split("\n\n");

        let seeds_1 = parts
            .next()
            .and_then(|s| s.strip_prefix("seeds: "))
            .map(|s| parse_number_list(s))
            .unwrap();

        let seeds_2 = seeds_1
            .chunks(2)
            .map(|nums| Range {
                start: nums[0],
                length: nums[1],
            })
            .collect();

        let maps = parts.map(Map::parse).collect();

        PuzzleInput {
            seeds_1,
            seeds_2,
            maps,
        }
    }
}

fn solve1(input: &str) -> u64 {
    let puzzle = PuzzleInput::parse(input);

    puzzle
        .seeds_1
        .into_iter()
        .map(|seed| {
            let mut result = seed;
            for map in &puzzle.maps {
                result = map.get(result);
            }
            result
        })
        .min()
        .unwrap()
}

fn solve2(input: &str) -> u64 {
    let puzzle = PuzzleInput::parse(input);

    puzzle
        .seeds_2
        .into_iter()
        .map(|seed_range| {
            let mut result = vec![seed_range];
            for map in &puzzle.maps {
                result = result
                    .into_iter()
                    .flat_map(|range| map.get_range(range))
                    .collect();
            }
            result
        })
        .flatten()
        .map(|r| r.start)
        .min()
        .unwrap()
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
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 35);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 46);
    }

    #[test]
    fn range() {
        let r1 = Range {
            start: 20,
            length: 30,
        };
        let r2 = Range {
            start: 15,
            length: 10,
        };
        assert_eq!(
            r1.overlap(&r2),
            Some(Range {
                start: 20,
                length: 5
            })
        );
    }
}
