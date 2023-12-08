use crate::day_8_part_1::*;

pub fn run(input: &str) -> u64 {
    eprintln!("Stop now! This will not work!");

    let (steps, hash_map) = parse_input(input);

    let mut counter = 0;
    let mut nodes = hash_map
        .values()
        .filter(|node| node.ends_with_a())
        .collect::<Vec<&Node>>();
    'outer: loop {
        for step in &steps {
            for node in nodes.iter_mut() {
                *node = hash_map.get(node.next(*step)).unwrap();
            }

            counter += 1;

            if nodes.iter().all(|node| node.ends_with_z()) {
                break 'outer;
            }
        }
    }

    counter
}

impl Node {
    fn ends_with_a(&self) -> bool {
        self.code.ends_with('A')
    }

    fn ends_with_z(&self) -> bool {
        self.code.ends_with('Z')
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
