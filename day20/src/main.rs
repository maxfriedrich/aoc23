use std::collections::{HashMap, HashSet, VecDeque};

enum Test {
    A(bool),
}

fn modify(test: &mut Test) {
    match test {
        Test::A(x) => {
            *x = true;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
enum Module<'a> {
    FlipFlop {
        name: &'a str,
        state: bool,
        outputs: Vec<&'a str>,
    },
    Conjunction {
        name: &'a str,
        most_recent_inputs: HashMap<&'a str, Pulse>,
        outputs: Vec<&'a str>,
    },
    Broadcaster {
        name: &'a str,
        outputs: Vec<&'a str>,
    },
    Output {
        name: &'a str,
    },
}

fn build(input: &str) -> HashMap<&str, Module> {
    #[derive(Hash, PartialEq, Eq)]
    enum ModuleType {
        FlipFlop,
        Conjunction,
        Broadcaster,
    }

    let mut module_inputs: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut modules_intermediate: HashMap<&str, (ModuleType, Vec<&str>)> = HashMap::new();
    let mut untyped_modules: HashSet<&str> = HashSet::new();

    for line in input.lines() {
        let (name_str, destinations_str) = line.split_once(" -> ").unwrap();
        let (module_name, module_type) = if name_str.starts_with('%') {
            (&name_str[1..], ModuleType::FlipFlop)
        } else if name_str.starts_with('&') {
            (&name_str[1..], ModuleType::Conjunction)
        } else if name_str == "broadcaster" {
            (name_str, ModuleType::Broadcaster)
        } else {
            panic!("invalid module name")
        };
        let destinations: Vec<&str> = destinations_str.split(", ").collect();

        for destination in &destinations {
            module_inputs
                .entry(destination)
                .or_insert(Vec::new())
                .push(module_name);

            if !modules_intermediate.contains_key(destination) {
                untyped_modules.insert(&destination);
            }
        }
        untyped_modules.remove(module_name);
        modules_intermediate.insert(module_name, (module_type, destinations));
    }

    let mut modules: HashMap<&str, Module> = modules_intermediate
        .iter()
        .map(|(module_name, (module_type, destinations))| {
            let module = match module_type {
                ModuleType::FlipFlop => Module::FlipFlop {
                    name: &module_name,
                    state: false,
                    outputs: destinations.clone(),
                },
                ModuleType::Conjunction => Module::Conjunction {
                    name: &module_name,
                    most_recent_inputs: module_inputs[module_name]
                        .iter()
                        .map(|in_name| (*in_name, Pulse::Low))
                        .collect(),
                    outputs: destinations.clone(),
                },
                ModuleType::Broadcaster => Module::Broadcaster {
                    name: &module_name,
                    outputs: destinations.clone(),
                },
            };
            (*module_name, module)
        })
        .collect();

    for module_name in untyped_modules {
        modules.insert(module_name, Module::Output { name: module_name });
    }

    modules
}

fn process(modules: &mut HashMap<&str, Module>) -> (usize, usize) {
    let mut lo = 1;
    let mut hi = 0;

    let mut todo: VecDeque<Vec<(&str, Pulse, &str)>> = VecDeque::new();
    todo.push_back(vec![(&"broadcaster", Pulse::Low, "button")]);

    while let Some(group) = todo.pop_front() {
        // dbg!(&group);

        for (module_name, pulse, source) in group {
            println!("{source} -{pulse:?}-> {module_name}");
            match pulse {
                Pulse::Low => lo += 1,
                Pulse::High => hi += 1,
            }

            // dbg!(modules.get_mut(module_name).unwrap());

            match modules.get_mut(module_name).unwrap() {
                Module::FlipFlop {
                    name,
                    state,
                    outputs,
                } => match (pulse, &state) {
                    (Pulse::High, _) => {}
                    (Pulse::Low, true) => {
                        *state = false;
                        todo.push_back(
                            outputs
                                .iter()
                                .copied()
                                .map(|o| (o, Pulse::Low, module_name))
                                .collect::<Vec<_>>(),
                        );
                    }
                    (Pulse::Low, false) => {
                        *state = true;
                        todo.push_back(
                            outputs
                                .iter()
                                .copied()
                                .map(|o| (o, Pulse::High, module_name))
                                .collect::<Vec<_>>(),
                        );
                    }
                },
                Module::Conjunction {
                    name,
                    ref mut most_recent_inputs,
                    outputs,
                } => {
                    *most_recent_inputs.get_mut(source).unwrap() = pulse;
                    dbg!(&name, &most_recent_inputs);
                    if most_recent_inputs.values().all(|p| p == &Pulse::High) {
                        todo.push_back(
                            outputs
                                .iter()
                                .copied()
                                .map(|o| (o, Pulse::Low, module_name))
                                .collect::<Vec<_>>(),
                        );
                    } else if most_recent_inputs.values().all(|p| p == &Pulse::Low) {
                        todo.push_back(
                            outputs
                                .iter()
                                .copied()
                                .map(|o| (o, Pulse::High, module_name))
                                .collect::<Vec<_>>(),
                        );
                    } else {
                        dbg!("no output from conjunction");
                    }
                }
                Module::Broadcaster { name, outputs } => {
                    todo.push_back(
                        outputs
                            .iter()
                            .copied()
                            .map(|o| (o, pulse, module_name))
                            .collect::<Vec<_>>(),
                    );
                }
                Module::Output { name } => {}
            }
        }
        // dbg!(&todo);
    }
    (lo, hi)
}

fn solve1(input: &str) -> usize {
    let mut modules = build(input);
    println!("{modules:?}");
    let mut hi = 0;
    let mut lo = 0;
    for i in 0..3 {
        println!("--- {i} ---");
        let (step_hi, step_lo) = process(&mut modules);
        hi += step_hi;
        lo += step_lo;
    }
    hi * lo
}

fn solve2(input: &str) -> u32 {
    todo!()
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

    //     const EXAMPLE: &str = "\
    // broadcaster -> a
    // %a -> inv, con
    // &inv -> b
    // %b -> con
    // &con -> output";

    #[test]
    fn example1() {
        let input = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!(solve1(input), 32000000);
    }

    #[test]
    fn example2() {
        let input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(solve2(input), 281);
    }
}
