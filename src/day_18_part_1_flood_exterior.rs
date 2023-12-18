use std::collections::HashMap;

use crate::clean_lines;

pub fn run(input: &str) -> i32 {
    let instructions = clean_lines(input)
        .map(Instruction::from)
        .collect::<Vec<Instruction>>();

    let (tiles, boundary) = dig(&instructions);

    count(&tiles, &boundary)
}

fn dig(instructions: &[Instruction]) -> (HashMap<Coords, Tile>, Boundary) {
    let mut tiles = HashMap::new();

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
                    tiles.insert(coords, Tile::Edge);
                });
            }
            'R' => {
                (0..i.distance).for_each(|_| {
                    coords = (coords.0 + 1, coords.1);
                    if coords.0 > x_upper {
                        x_upper = coords.0;
                    }
                    tiles.insert(coords, Tile::Edge);
                });
            }
            'D' => {
                (0..i.distance).for_each(|_| {
                    coords = (coords.0, coords.1 + 1);
                    if coords.1 > y_upper {
                        y_upper = coords.1;
                    }
                    tiles.insert(coords, Tile::Edge);
                });
            }
            'L' => {
                (0..i.distance).for_each(|_| {
                    coords = (coords.0 - 1, coords.1);
                    if coords.0 < x_lower {
                        x_lower = coords.0;
                    }
                    tiles.insert(coords, Tile::Edge);
                });
            }
            _ => panic!("Unexpected direction {} in {:?}", i.direction, i),
        };
    });

    (
        tiles,
        Boundary {
            y_lower,
            y_upper,
            x_lower,
            x_upper,
        },
    )
}

fn count(tiles: &HashMap<Coords, Tile>, boundary: &Boundary) -> i32 {
    // Expand the border by 1.
    let boundary = Boundary {
        y_lower: boundary.y_lower - 1,
        y_upper: boundary.y_upper + 1,
        x_lower: boundary.x_lower - 1,
        x_upper: boundary.x_upper + 1,
    };

    // Flood fill the "exterior".
    let mut tiles = tiles.clone();
    flood_fill(&mut tiles, &boundary, (boundary.x_lower, boundary.y_lower));

    ((boundary.y_upper - boundary.y_lower + 1) * (boundary.x_upper - boundary.x_lower + 1))
        - tiles.values().filter(|t| **t == Tile::External).count() as i32
}

fn flood_fill(tiles: &mut HashMap<Coords, Tile>, boundary: &Boundary, seed: Coords) {
    let mut queue = vec![seed];

    while let Some(coords) = queue.pop() {
        neighbors_8(&coords).iter().for_each(|c| {
            if boundary.contains(c) && tiles.get(c).is_none() {
                tiles.insert(*c, Tile::External);
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

#[derive(Clone, PartialEq)]
enum Tile {
    Edge,
    External,
}

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

struct Boundary {
    y_lower: i32,
    y_upper: i32,
    x_lower: i32,
    x_upper: i32,
}

impl Boundary {
    fn contains(&self, coords: &Coords) -> bool {
        (self.y_lower..=self.y_upper).contains(&coords.1)
            && (self.x_lower..=self.x_upper).contains(&coords.0)
    }
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
