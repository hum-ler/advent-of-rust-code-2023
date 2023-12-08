use std::collections::HashMap;

use crate::clean_lines;

pub fn run(input: &str) -> u64 {
    // This following is written after looking up other people's solutions. The general strategy
    // seems to be using the trick that, for this particular set of input, each XXA -> XXZ forms a
    // cyclic graph, and that it loops around perfectly after the first XXZ. So, if we can find the
    // length of the cycle for each XXA, we can then calculate the LCM to figure out when all the
    // XXZs will first coincide.

    let (steps, hash_map) = parse_input(input);

    let codes = hash_map
        .keys()
        .filter(|code| code_ends_with_a(code))
        .map(|code| *code)
        .collect::<Vec<&str>>();

    let cycle_lengths = codes
        .iter()
        .map(|code| {
            let mut counter = 0;
            let mut current_code = *code;

            'outer: loop {
                for step in &steps {
                    current_code = hash_map.get(current_code).unwrap()[*step];
                    counter += 1;
                    if code_ends_with_z(current_code) {
                        break 'outer;
                    }
                }
            }

            counter
        })
        .collect::<Vec<u64>>();

    cycle_lengths
        .into_iter()
        .reduce(|acc, length| lcm(acc, length))
        .unwrap()
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

// Copied from https://www.hackertouch.com/least-common-multiple-in-rust.html.
fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

// Copied from https://www.hackertouch.com/least-common-multiple-in-rust.html.
fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
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
