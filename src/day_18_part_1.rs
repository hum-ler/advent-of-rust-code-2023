use std::collections::HashSet;

use crate::clean_lines;

pub fn run(input: &str) -> usize {
    let instructions = clean_lines(input)
        .map(Instruction::from)
        .collect::<Vec<Instruction>>();

    let (mut holes, bounding_box) = dig(&instructions);

    // Find a suitable seed for flood fill.
    let mut seed = (0, 0);
    'outer: for y in bounding_box.top_left.1 + 1..bounding_box.bottom_right.1 {
        for x in bounding_box.top_left.0..bounding_box.bottom_right.0 - 1 {
            if holes.contains(&(x, y)) {
                if !holes.contains(&(x + 1, y)) {
                    seed = (x + 1, y);
                    break 'outer;
                } else {
                    continue 'outer;
                }
            }
        }
    }
    flood_fill(&mut holes, seed);

    holes.len()
}

fn dig(instructions: &[Instruction]) -> (HashSet<Coords>, BoundingBox) {
    let mut holes = HashSet::new();

    let mut y_lower = i32::MAX;
    let mut y_upper = i32::MIN;
    let mut x_lower = i32::MAX;
    let mut x_upper = i32::MIN;

    // Dig the edges.
    let mut coords = (0, 0);
    instructions.iter().for_each(|i| {
        match i.direction {
            'U' => {
                (0..i.distance).for_each(|_| {
                    coords = (coords.0, coords.1 - 1);
                    if coords.1 < y_lower {
                        y_lower = coords.1;
                    }
                    holes.insert(coords);
                });
            }
            'R' => {
                (0..i.distance).for_each(|_| {
                    coords = (coords.0 + 1, coords.1);
                    if coords.0 > x_upper {
                        x_upper = coords.0;
                    }
                    holes.insert(coords);
                });
            }
            'D' => {
                (0..i.distance).for_each(|_| {
                    coords = (coords.0, coords.1 + 1);
                    if coords.1 > y_upper {
                        y_upper = coords.1;
                    }
                    holes.insert(coords);
                });
            }
            'L' => {
                (0..i.distance).for_each(|_| {
                    coords = (coords.0 - 1, coords.1);
                    if coords.0 < x_lower {
                        x_lower = coords.0;
                    }
                    holes.insert(coords);
                });
            }
            _ => panic!("Unexpected direction {} in {:?}", i.direction, i),
        };
    });

    (
        holes,
        BoundingBox {
            top_left: (x_lower, y_lower),
            bottom_right: (x_upper, x_upper),
        },
    )
}

fn flood_fill(holes: &mut HashSet<Coords>, seed: Coords) {
    let mut queue = vec![seed];

    while let Some(coords) = queue.pop() {
        neighbors_8(&coords).iter().for_each(|c| {
            if holes.get(c).is_none() {
                holes.insert(*c);
                queue.push(*c);
            }
        });
    }
}

fn neighbors_8(coords: &Coords) -> Vec<Coords> {
    vec![
        (coords.0 - 1, coords.1 - 1),
        (coords.0, coords.1 - 1),
        (coords.0 + 1, coords.1 - 1),
        (coords.0 - 1, coords.1),
        (coords.0 + 1, coords.1),
        (coords.0 - 1, coords.1 + 1),
        (coords.0, coords.1 + 1),
        (coords.0 + 1, coords.1 + 1),
    ]
}

type Coords = (i32, i32);

#[derive(Debug)]
struct Instruction {
    direction: char,
    distance: i32,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Instruction {
        let tokens = value.split(' ').collect::<Vec<&str>>();
        if tokens.len() != 3 {
            panic!("Unexpected input {value}");
        }

        Self {
            direction: tokens[0].parse::<char>().unwrap(),
            distance: tokens[1].parse::<i32>().unwrap(),
        }
    }
}

struct BoundingBox {
    top_left: Coords,
    bottom_right: Coords,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
        ";

        assert_eq!(run(input), 62);
    }
}
