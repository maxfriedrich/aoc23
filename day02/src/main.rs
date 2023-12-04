use std::{cmp::max, iter::Sum};

#[derive(Clone, Copy)]
struct CubeSet {
    red: i32,
    blue: i32,
    green: i32,
}

impl Sum for CubeSet {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, d| CubeSet {
            red: acc.red + d.red,
            blue: acc.blue + d.blue,
            green: acc.green + d.green,
        })
        .unwrap()
    }
}

impl CubeSet {
    fn parse(input: &str) -> CubeSet {
        input
            .split(", ")
            .map(|part| {
                let (n_str, color_str) = part.split_once(' ').unwrap();
                let n: i32 = n_str.parse().unwrap();

                match color_str {
                    "red" => CubeSet {
                        red: n,
                        blue: 0,
                        green: 0,
                    },
                    "blue" => CubeSet {
                        red: 0,
                        blue: n,
                        green: 0,
                    },
                    "green" => CubeSet {
                        red: 0,
                        blue: 0,
                        green: n,
                    },
                    _ => panic!(),
                }
            })
            .sum()
    }

    fn is_possible(&self, config: &CubeSet) -> bool {
        self.red <= config.red && self.green <= config.green && self.blue <= config.blue
    }

    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

struct Game {
    id: i32,
    draws: Vec<CubeSet>,
}

impl Game {
    fn parse(input: &str) -> Game {
        let (id_part, draws_part) = input.split_once(": ").unwrap();
        let id: i32 = id_part.strip_prefix("Game ").unwrap().parse().unwrap();

        let draws = draws_part.split("; ").map(CubeSet::parse).collect();

        Game {
            id: id,
            draws: draws,
        }
    }

    fn is_possible(&self, config: &CubeSet) -> bool {
        self.draws.iter().all(|draw| draw.is_possible(config))
    }

    fn min_config(&self) -> CubeSet {
        self.draws
            .iter()
            .map(|d| d.to_owned())
            .reduce(|acc, d| CubeSet {
                red: max(acc.red, d.red),
                blue: max(acc.blue, d.blue),
                green: max(acc.green, d.green),
            })
            .unwrap()
            .clone()
    }
}

fn solve1(input: &str, config: CubeSet) -> i32 {
    let games = input.split('\n').map(Game::parse);
    games
        .filter(|game| game.is_possible(&config))
        .map(|game| game.id)
        .sum()
}

fn solve2(input: &str) -> i32 {
    let games = input.split('\n').map(Game::parse);
    games.map(|g| g.min_config()).map(|c| c.power()).sum()
}

fn main() {
    let input = include_str!("input.txt");
    println!(
        "{}",
        solve1(
            input,
            CubeSet {
                red: 12,
                green: 13,
                blue: 14
            }
        )
    );
    println!("{}", solve2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn example1() {
        assert_eq!(
            solve1(
                EXAMPLE,
                CubeSet {
                    red: 12,
                    green: 13,
                    blue: 14
                }
            ),
            8
        );
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 2286);
    }
}
