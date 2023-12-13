use std::collections::HashMap;

use crate::clean_lines;

pub fn run(input: &str) -> u64 {
    eprintln!("Stop now! This will not work!");

    let (steps, hash_map) = parse_input(input);

    let mut counter = 0;
    let mut codes = hash_map
        .keys()
        .filter(|code| code_ends_with_a(code))
        .copied()
        .collect::<Vec<&str>>();

    'outer: loop {
        for step in &steps {
            codes
                .iter_mut()
                .for_each(|code| *code = hash_map.get(code).unwrap()[*step]);

            counter += 1;

            if is_end(&codes) {
                break 'outer;
            }
        }
    }

    counter
}

fn parse_input(input: &str) -> (Vec<usize>, HashMap<&str, [&str; 2]>) {
    let lines = clean_lines(input).collect::<Vec<&str>>();

    let steps = parse_instruction(lines[0]);

    let mut nodes = HashMap::new();
    lines[1..]
        .iter()
        .map(|line| parse_nodes(line))
        .for_each(|(code, node)| {
            nodes.insert(code, node);
        });

    (steps, nodes)
}

// Returns a Vec of usize where:
// - 0 represents left
// - 1 represents right
fn parse_instruction(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!(),
        })
        .collect()
}

// Returns a tuple of (code, [left, right]).
fn parse_nodes(input: &str) -> (&str, [&str; 2]) {
    (&input[0..3], [&input[7..10], &input[12..15]])
}

fn code_ends_with_a(code: &str) -> bool {
    code.ends_with('A')
}

fn code_ends_with_z(code: &str) -> bool {
    code.ends_with('Z')
}

fn is_end(codes: &[&str]) -> bool {
    codes.iter().all(|code| code_ends_with_z(code))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        ";

        assert_eq!(run(input), 6);
    }
}
