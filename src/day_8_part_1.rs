use std::collections::HashMap;

use crate::clean_lines;

pub fn run(input: &str) -> u32 {
    let (steps, nodes) = parse_input(input);

    let mut counter = 0;
    let mut node = nodes.get(&String::from("AAA")).unwrap(); // initialize
    'outer: loop {
        for step in &steps {
            node = nodes.get(node.next(*step)).unwrap();
            counter += 1;
            if node.is_end() {
                break 'outer;
            }
        }
    }

    counter
}

pub(crate) fn parse_input(input: &str) -> (Vec<Step>, HashMap<String, Node>) {
    let lines = clean_lines(input).collect::<Vec<&str>>();

    let steps = parse_instruction(lines[0]);

    let mut nodes = HashMap::new();
    lines[1..]
        .iter()
        .map(|line| parse_node(line))
        .for_each(|node| {
            nodes.insert(node.code.to_owned(), node);
        });

    (steps, nodes)
}

fn parse_instruction(input: &str) -> Vec<Step> {
    input.chars().map(Step::from).collect()
}

fn parse_node(input: &str) -> Node {
    if input.len() != 16 {
        panic!("Cannot parse Node {input}");
    }

    Node::new(&input[0..3], &input[7..10], &input[12..15])
}

#[derive(Clone, Copy)]
pub(crate) enum Step {
    Left,
    Right,
}

impl From<char> for Step {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Cannot parse step {value}"),
        }
    }
}

pub(crate) struct Node {
    pub code: String,
    left: String,
    right: String,
}

impl Node {
    fn new(code: &str, left: &str, right: &str) -> Self {
        Self {
            code: String::from(code),
            left: String::from(left),
            right: String::from(right),
        }
    }

    fn is_end(&self) -> bool {
        self.code.as_str() == "ZZZ"
    }

    pub(crate) fn next(&self, step: Step) -> &str {
        match step {
            Step::Left => &self.left,
            Step::Right => &self.right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input_1 = r"
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        ";

        assert_eq!(run(input_1), 2);

        let input_2 = r"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        ";

        assert_eq!(run(input_2), 6);
    }
}
