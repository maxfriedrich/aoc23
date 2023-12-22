use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn parse(input: char) -> Self {
        match input {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!("Invalid category: {}", input),
        }
    }

    fn index(&self) -> usize {
        match self {
            Category::X => 0,
            Category::M => 1,
            Category::A => 2,
            Category::S => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    GreaterThan,
    LessThan,
}

impl Op {
    fn parse(input: char) -> Self {
        match input {
            '>' => Self::GreaterThan,
            '<' => Self::LessThan,
            _ => panic!("invalid op: {}", input),
        }
    }
}

#[derive(Debug)]
enum Rule<'a> {
    ShortCircuit {
        destination: &'a str,
    },
    Comparison {
        category: Category,
        op: Op,
        value: u32,
        destination: &'a str,
    },
}

impl<'a> Rule<'a> {
    fn parse(input: &'a str) -> Self {
        if input.contains(':') {
            let category = Category::parse(input.chars().nth(0).unwrap());
            let op = Op::parse(input.chars().nth(1).unwrap());
            let (value_str, destination) = input[2..].split_once(':').unwrap();
            Self::Comparison {
                category,
                op,
                value: value_str.parse().unwrap(),
                destination,
            }
        } else {
            Self::ShortCircuit { destination: input }
        }
    }

    fn evaluate_part(&self, part: &Part) -> Option<&'a str> {
        match self {
            Self::ShortCircuit { destination } => Some(destination),
            Self::Comparison {
                category,
                op,
                value,
                destination,
            } => {
                let part_value = part.categories[category.index()];
                let part_matches = match op {
                    Op::GreaterThan => part_value > *value,
                    Op::LessThan => part_value < *value,
                };
                if part_matches {
                    Some(destination)
                } else {
                    None
                }
            }
        }
    }

    fn evaluate_parts(&self, parts: &Parts) -> HashMap<Parts, Option<&'a str>> {
        let mut result: HashMap<Parts, Option<&'a str>> = HashMap::new();

        match self {
            Self::ShortCircuit { destination } => {
                result.insert(*parts, Some(destination));
            }
            Self::Comparison {
                category,
                op,
                value,
                destination,
            } => {
                let category_index = category.index();
                let part_value_range = parts.categories[category_index];

                match (op, part_value_range) {
                    (Op::GreaterThan, CategoryRange { start, end }) => {
                        if &start > value {
                            result.insert(*parts, Some(destination));
                        } else if &end > value {
                            result.insert(
                                parts.with_range_replaced(
                                    category_index,
                                    CategoryRange {
                                        start: value + 1,
                                        end,
                                    },
                                ),
                                Some(destination),
                            );
                            result.insert(
                                parts.with_range_replaced(
                                    category_index,
                                    CategoryRange {
                                        start,
                                        end: value + 1,
                                    },
                                ),
                                None,
                            );
                        } else {
                            result.insert(*parts, None);
                        }
                    }
                    (Op::LessThan, CategoryRange { start, end }) => {
                        if &end < value {
                            result.insert(*parts, Some(destination));
                        } else if &start < value {
                            result.insert(
                                parts.with_range_replaced(
                                    category_index,
                                    CategoryRange { start, end: *value },
                                ),
                                Some(destination),
                            );
                            result.insert(
                                parts.with_range_replaced(
                                    category_index,
                                    CategoryRange { start: *value, end },
                                ),
                                None,
                            );
                        } else {
                            result.insert(*parts, None);
                        }
                    }
                };
            }
        }
        result
    }
}

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn parse(input: &'a str) -> Self {
        let (name, rest) = input.split_once('{').unwrap();
        let rules = rest
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(Rule::parse)
            .collect();

        Self { name, rules }
    }

    fn evaluate_part(&self, part: &Part) -> &'a str {
        for rule in &self.rules {
            if let Some(id) = rule.evaluate_part(part) {
                return id;
            }
        }
        panic!("Workflow should have catch-all rule")
    }

    fn evaluate_parts(&self, parts: &Parts) -> HashMap<Parts, &'a str> {
        let mut result: HashMap<Parts, &'a str> = HashMap::new();
        let mut unresolved: Vec<Parts> = vec![*parts];

        for rule in &self.rules {
            let todo = unresolved.clone();
            unresolved = vec![];
            for parts in todo {
                for (p, destination_opt) in rule.evaluate_parts(&parts) {
                    match destination_opt {
                        Some(destination) => {
                            result.insert(p, destination);
                        }
                        None => {
                            unresolved.push(p);
                        }
                    }
                }
            }
        }
        result
    }
}

#[derive(Debug)]
struct Part {
    categories: [u32; 4],
}

impl Part {
    fn parse(input: &str) -> Self {
        let values: HashMap<char, u32> = input
            .strip_prefix('{')
            .and_then(|x| x.strip_suffix('}'))
            .unwrap()
            .split(',')
            .map(|spec| (spec.chars().nth(0).unwrap(), spec[2..].parse().unwrap()))
            .collect();

        Self {
            categories: [values[&'x'], values[&'m'], values[&'a'], values[&'s']],
        }
    }
}

#[derive(Debug)]
struct PuzzleInput<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
    parts: Vec<Part>,
}

impl<'a> PuzzleInput<'a> {
    fn parse(input: &'a str) -> Self {
        let (workflows_part, parts_part) = input.split_once("\n\n").unwrap();
        let workflows = workflows_part
            .lines()
            .map(|line| {
                let workflow = Workflow::parse(line);
                (workflow.name, workflow)
            })
            .collect();
        let parts = parts_part.lines().map(Part::parse).collect();

        Self { workflows, parts }
    }
}

fn is_part_accepted(part: &Part, workflows: &HashMap<&str, Workflow>) -> bool {
    let mut current_workflow_id = "in";
    while current_workflow_id != "R" && current_workflow_id != "A" {
        current_workflow_id = workflows[current_workflow_id].evaluate_part(part);
    }
    current_workflow_id == "A"
}

fn solve1(input: &str) -> u32 {
    let input = PuzzleInput::parse(input);
    // dbg!(&input);

    input
        .parts
        .into_iter()
        .filter_map(|part| {
            if is_part_accepted(&part, &input.workflows) {
                Some(part)
            } else {
                None
            }
        })
        .map(|part| part.categories.iter().sum::<u32>())
        .sum()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct CategoryRange {
    start: u32,
    end: u32,
}

impl CategoryRange {
    fn len(&self) -> usize {
        (self.end - self.start).try_into().unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Parts {
    categories: [CategoryRange; 4],
}

impl Parts {
    fn with_range_replaced(&self, index: usize, new_range: CategoryRange) -> Self {
        let mut categories = self.categories;
        *categories.get_mut(index).unwrap() = new_range;

        Self { categories }
    }

    fn num_parts(&self) -> usize {
        self.categories.iter().map(|r| r.len()).product()
    }
}

fn num_accepted(parts: &Parts, workflows: &HashMap<&str, Workflow>) -> usize {
    let mut result = 0;
    let mut todo = vec![(*parts, "in")];

    while let Some((parts, workflow_id)) = todo.pop() {
        let workflow_result = workflows[workflow_id].evaluate_parts(&parts);
        // dbg!(&workflow_result);
        for (parts_subset, next_workflow_id) in workflow_result {
            if next_workflow_id == "A" {
                result += parts_subset.num_parts();
            } else if next_workflow_id != "R" {
                todo.push((parts_subset, next_workflow_id));
            }
        }
    }

    result
}

fn solve2(input: &str) -> usize {
    let input = PuzzleInput::parse(input);
    let parts = Parts {
        categories: [CategoryRange {
            start: 1,
            end: 4001,
        }; 4],
    };

    num_accepted(&parts, &input.workflows)
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
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 19114);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 167409079868000);
    }
}
