use std::collections::HashMap;

type NodeId = [char; 3];

fn parse_node_id(input: &str) -> NodeId {
    let mut chars = input.chars();
    [
        chars.next().unwrap(),
        chars.next().unwrap(),
        chars.next().unwrap(),
    ]
}

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn parse(input: char) -> Direction {
        match input {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!(),
        }
    }

    fn value(&self) -> usize {
        match self {
            Self::Left => 0,
            Self::Right => 1,
        }
    }
}

struct Node {
    #[allow(dead_code)]
    id: NodeId,
    children: [NodeId; 2],
}

struct Network {
    nodes: HashMap<NodeId, Node>,
}

impl Network {
    fn parse(input: &str) -> Network {
        let mut nodes = HashMap::new();
        for line in input.lines() {
            let (id_part, lr_part) = line.split_once(" = ").unwrap();

            let id = parse_node_id(id_part);
            let (l_part, r_part) = lr_part
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .and_then(|s| s.split_once(", "))
                .unwrap();
            let left = parse_node_id(l_part);
            let right = parse_node_id(r_part);

            nodes.insert(
                id,
                Node {
                    id,
                    children: [left, right],
                },
            );
        }

        Network { nodes }
    }
}

struct PuzzleInput {
    instructions: Vec<Direction>,
    network: Network,
}

impl PuzzleInput {
    fn parse(input: &str) -> PuzzleInput {
        let (instructions_part, network_part) = input.split_once("\n\n").unwrap();
        let instructions = instructions_part.chars().map(Direction::parse).collect();
        let network = Network::parse(network_part);

        PuzzleInput {
            instructions,
            network,
        }
    }
}

fn num_steps_to_node(
    start_node_id: &NodeId,
    network: &Network,
    instructions: &[Direction],
    node_match_fn: impl Fn(&NodeId) -> bool,
) -> u64 {
    let mut current_node_id = start_node_id;
    let mut num_steps = 0;

    let mut instructions = instructions.iter().cycle();

    while !node_match_fn(current_node_id) {
        let direction = instructions.next().unwrap();
        let current_node = network.nodes.get(current_node_id).unwrap();

        current_node_id = current_node.children.get(direction.value()).unwrap();
        num_steps += 1;
    }

    num_steps
}

fn solve1(input: &str) -> u64 {
    let puzzle = PuzzleInput::parse(input);

    num_steps_to_node(
        &['A', 'A', 'A'],
        &puzzle.network,
        &puzzle.instructions,
        |node_id| node_id == &['Z', 'Z', 'Z'],
    )
}

// thanks for the help ChatGPT!
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn solve2(input: &str) -> u64 {
    let puzzle = PuzzleInput::parse(input);

    puzzle
        .network
        .nodes
        .keys()
        .filter(|k| k.get(2).unwrap() == &'A')
        .map(|a_node| {
            num_steps_to_node(a_node, &puzzle.network, &puzzle.instructions, |node_id| {
                node_id.get(2).unwrap() == &'Z'
            })
        })
        .fold(1, lcm)
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
    fn example1() {
        let input = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(solve1(input), 2);
    }

    #[test]
    fn example1b() {
        let input = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(solve1(input), 6);
    }

    #[test]
    fn example2() {
        let input = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(solve2(input), 6);
    }
}
