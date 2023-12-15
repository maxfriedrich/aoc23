fn hash_algorithm(input: &str) -> u32 {
    let mut current_value = 0;
    for char in input.chars() {
        current_value += char as u32;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

#[derive(Debug, Clone)]
enum Operation {
    Remove { label: String },
    Set { label: String, focal_length: u8 },
}

impl Operation {
    fn parse(input: &str) -> Self {
        if let Some(label) = input.strip_suffix('-') {
            Self::Remove {
                label: label.to_string(),
            }
        } else {
            Self::Set {
                label: input[..input.len() - 2].to_string(),
                focal_length: input[input.len() - 1..].parse().unwrap(),
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u8,
}

fn solve1(input: &str) -> u32 {
    input.split(',').map(hash_algorithm).sum()
}

fn solve2(input: &str) -> usize {
    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];
    let operations = input.split(',').map(Operation::parse);
    for op in operations {
        match op {
            Operation::Remove { label } => {
                let op_box = &mut boxes.get_mut(hash_algorithm(&label) as usize).unwrap();
                if let Some(position) = &op_box.iter().position(|lens| lens.label == label) {
                    op_box.remove(*position);
                }
            }
            Operation::Set {
                label,
                focal_length,
            } => {
                let op_box = &mut boxes[hash_algorithm(&label) as usize];
                if let Some(position) = &op_box.iter().position(|lens| lens.label == label) {
                    let lens = op_box.get_mut(*position).unwrap();
                    lens.focal_length = focal_length
                } else {
                    op_box.push(Lens {
                        label,
                        focal_length,
                    });
                }
            }
        }
    }
    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_num_zero_indexed, b)| {
            b.iter().enumerate().map(move |(slot_num_zero_indexed, l)| {
                (box_num_zero_indexed + 1) * (slot_num_zero_indexed + 1) * (l.focal_length as usize)
            })
        })
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

    #[test]
    fn test_hash_algorithm() {
        assert_eq!(hash_algorithm("HASH"), 52);
    }

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 1320);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 145);
    }
}
