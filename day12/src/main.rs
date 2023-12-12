use cached::proc_macro::cached;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum SpringCondition {
    Operational,
    Damaged,
    Unknown,
}

impl SpringCondition {
    fn parse(input: char) -> Self {
        match input {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!(),
        }
    }

    #[allow(dead_code)]
    fn char(&self) -> char {
        match self {
            Self::Operational => '.',
            Self::Damaged => '#',
            Self::Unknown => '?',
        }
    }
}

struct Spring {
    conditions: Vec<SpringCondition>,
    damaged_segments: Vec<usize>,
}

impl Spring {
    fn parse(input: &str) -> Self {
        let (conditions_str, segments_str) = input.split_once(' ').unwrap();
        let conditions = conditions_str.chars().map(SpringCondition::parse).collect();
        let damaged_segments = segments_str
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();

        Self {
            conditions,
            damaged_segments,
        }
    }

    fn parse_folded(input: &str) -> Self {
        let raw_spring = Self::parse(input);
        let mut conditions = Vec::new();
        let mut damaged_segments = Vec::new();
        for i in 0..5 {
            conditions.extend(raw_spring.conditions.iter());
            if i < 4 {
                conditions.push(SpringCondition::Unknown);
            }
            damaged_segments.extend(raw_spring.damaged_segments.iter());
        }

        Self {
            conditions,
            damaged_segments,
        }
    }
}

fn condition_segments(conditions: &[SpringCondition]) -> Vec<usize> {
    let mut in_segment = false;
    let mut segment_size = 0;
    let mut segments = Vec::new();
    for cond in conditions {
        match (cond, in_segment) {
            (SpringCondition::Damaged, true) => segment_size += 1,
            (SpringCondition::Damaged, false) => {
                in_segment = true;
                segment_size += 1;
            }
            (SpringCondition::Operational, true) => {
                segments.push(segment_size);
                in_segment = false;
                segment_size = 0;
            }
            (SpringCondition::Operational, false) => {}
            _ => panic!("condition should not be unknown here"),
        }
    }
    if in_segment {
        segments.push(segment_size);
    }
    segments
}

#[derive(Debug)]
struct ConditionStats {
    #[allow(dead_code)]
    num_operational: usize,
    num_damaged: usize,
    num_unknown: usize,
    fixed_until: usize,
}

impl ConditionStats {
    fn from(conditions: &[SpringCondition]) -> Self {
        let mut num_operational = 0;
        let mut num_damaged = 0;
        let mut num_unknown = 0;
        let mut fixed_until = 0;

        for (i, cond) in conditions.iter().enumerate() {
            match cond {
                SpringCondition::Operational => {
                    num_operational += 1;
                    if num_unknown == 0 {
                        fixed_until = i
                    };
                }
                SpringCondition::Damaged => num_damaged += 1,
                SpringCondition::Unknown => num_unknown += 1,
            }
        }

        Self {
            num_operational,
            num_damaged,
            num_unknown,
            fixed_until,
        }
    }
}

#[cached]
// memoization with "cached" requires taking ownership of parameters
fn n_arrangements(conditions: Vec<SpringCondition>, segments: Vec<usize>) -> usize {
    let stats = ConditionStats::from(&conditions);
    if stats.num_unknown == 0 {
        if condition_segments(&conditions) == segments {
            return 1;
        } else {
            return 0;
        }
    }

    let target_damaged = segments.iter().sum();

    if stats.num_damaged > target_damaged {
        return 0;
    }
    if stats.num_damaged + stats.num_unknown < target_damaged {
        return 0;
    }

    let (fixed_conditions, remaining_conditions) = conditions.split_at(stats.fixed_until);
    let fixed_segments = condition_segments(fixed_conditions);

    if fixed_segments.len() > segments.len() {
        return 0;
    }

    let (done_segments, todo_segments) = segments.split_at(fixed_segments.len());
    if done_segments != fixed_segments {
        return 0;
    }

    let first_unknown = remaining_conditions
        .iter()
        .position(|&c| c == SpringCondition::Unknown)
        .unwrap();
    let mut s1 = Vec::from(remaining_conditions);
    let mut s2 = Vec::from(remaining_conditions);
    s1[first_unknown] = SpringCondition::Damaged;
    s2[first_unknown] = SpringCondition::Operational;

    n_arrangements(s1, todo_segments.to_vec()) + n_arrangements(s2, todo_segments.to_vec())
}

fn solve1(input: &str) -> usize {
    let springs = input.lines().map(Spring::parse);
    springs
        .map(|s| n_arrangements(s.conditions, s.damaged_segments))
        .sum()
}

fn solve2(input: &str) -> usize {
    let springs = input.lines().map(Spring::parse_folded);
    springs
        .map(|s| n_arrangements(s.conditions, s.damaged_segments))
        .sum()
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
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 21);
    }

    #[test]
    fn test_conditions_match_segments() {
        let s1 = Spring::parse("#.#.### 1,1,3");
        assert_eq!(condition_segments(&s1.conditions), s1.damaged_segments);

        let s2 = Spring::parse(".#...#....###. 1,1,3");
        assert_eq!(condition_segments(&s2.conditions), s2.damaged_segments);

        let s3 = Spring::parse("#....######..#####. 1,6,5,1");
        assert_ne!(condition_segments(&s3.conditions), s3.damaged_segments);
    }

    #[test]
    fn parse_folded() {
        let s = Spring::parse_folded("???.### 1,1,3");
        assert_eq!(
            s.conditions
                .iter()
                .map(|c| c.char().to_string())
                .collect::<Vec<_>>()
                .join(""),
            "???.###????.###????.###????.###????.###"
        );

        assert_eq!(
            s.damaged_segments,
            vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 525152);
    }
}
