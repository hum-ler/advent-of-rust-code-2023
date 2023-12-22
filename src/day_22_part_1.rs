use std::collections::{HashMap, HashSet};

use crate::clean_lines;

pub fn run(input: &str) -> usize {
    let mut bricks = clean_lines(input).map(parse_line).collect::<Vec<Brick>>();
    bricks.sort_by(|a, b| a.position.2.cmp(&b.position.2));

    let bricks = settle(bricks);

    bricks
        .iter()
        .filter(|b| b.supports.iter().all(|s| bricks[*s].supported_by.len() > 1))
        .count()
}

pub(crate) fn settle(bricks: Vec<Brick>) -> Vec<Brick> {
    let mut settled_bricks = vec![];
    let mut occupancies = HashMap::new();

    for (id, mut brick) in bricks.into_iter().enumerate() {
        // Update the layer.
        while !(brick.position.2 == 1
            || brick
                .footprint()
                .iter()
                .any(|f| occupancies.contains_key(f)))
        {
            brick.position.2 -= 1;
        }

        brick.occupancy().into_iter().for_each(|o| {
            occupancies.insert(o, id);
        });

        // Update the supported_by.
        brick
            .footprint()
            .iter()
            .filter_map(|f| occupancies.get(f))
            .for_each(|s| {
                brick.supported_by.insert(*s);
            });

        settled_bricks.push(brick);
    }

    // Update the supports.
    for (id, brick) in settled_bricks.clone().iter().enumerate() {
        for support_brick_id in brick.supported_by.clone() {
            settled_bricks[support_brick_id].supports.insert(id);
        }
    }

    settled_bricks
}

pub(crate) fn parse_line(input: &str) -> Brick {
    let (left_end_part, right_end_part) = input.split_once('~').unwrap();
    let left_end = parse_end(left_end_part);
    let right_end = parse_end(right_end_part);

    Brick {
        position: (
            left_end.0.min(right_end.0),
            left_end.1.min(right_end.1),
            left_end.2.min(right_end.2),
        ),
        size: (
            left_end.0.abs_diff(right_end.0) + 1,
            left_end.1.abs_diff(right_end.1) + 1,
            left_end.2.abs_diff(right_end.2) + 1,
        ),
        supports: HashSet::new(),
        supported_by: HashSet::new(),
    }
}

fn parse_end(input: &str) -> Coords {
    // Assumption: x and y can only be single-digit.
    (
        input[0..1].parse::<usize>().unwrap(),
        input[2..3].parse::<usize>().unwrap(),
        input[4..].parse::<usize>().unwrap(),
    )
}

type Coords = (usize, usize, usize);

#[derive(Clone)]
pub(crate) struct Brick {
    pub(crate) position: Coords,
    size: Coords,
    pub(crate) supports: HashSet<usize>,
    pub(crate) supported_by: HashSet<usize>,
}

impl Brick {
    fn occupancy(&self) -> Vec<Coords> {
        match self.size {
            (x, 1, 1) => (self.position.0..self.position.0 + x)
                .map(|x| (x, self.position.1, self.position.2))
                .collect(),
            (1, y, 1) => (self.position.1..self.position.1 + y)
                .map(|y| (self.position.0, y, self.position.2))
                .collect(),
            (1, 1, z) => (self.position.2..self.position.2 + z)
                .map(|z| (self.position.0, self.position.1, z))
                .collect(),
            _ => unreachable!(),
        }
    }

    fn footprint(&self) -> Vec<Coords> {
        match self.size {
            (x, 1, 1) => (self.position.0..self.position.0 + x)
                .map(|x| (x, self.position.1, self.position.2 - 1))
                .collect(),
            (1, y, 1) => (self.position.1..self.position.1 + y)
                .map(|y| (self.position.0, y, self.position.2 - 1))
                .collect(),
            (1, 1, _z) => vec![(self.position.0, self.position.1, self.position.2 - 1)],
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
            1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9
        ";

        assert_eq!(run(input), 5);
    }
}
