use std::collections::HashMap;

use pathfinding::prelude::dijkstra;

use crate::{clean_lines, day_17_part_1::*};

pub fn run(input: &str) -> u32 {
    let input = clean_lines(input)
        .map(str::chars)
        .map(|row| row.collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let column_count = input[0].len();
    let row_count = input.len();

    let mut weights = HashMap::<Coords, u32>::new();
    input.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            weights.insert((x, y), c.to_digit(10).unwrap());
        });
    });

    dijkstra(
        &Vertex::new((0, 0), vec![]),
        |v| successors(v, &weights, column_count, row_count),
        |v| v.coords == (column_count - 1, row_count - 1),
    )
    .unwrap()
    .1
}

fn successors(
    vertex: &Vertex,
    weights: &HashMap<Coords, u32>,
    column_count: usize,
    row_count: usize,
) -> Vec<(Vertex, u32)> {
    let mut next_vertices = vec![];

    if vertex.coords.0 > 0
        && permitted_next_directions(&vertex.last_directions).contains(&Direction::Left)
    {
        let coords = (vertex.coords.0 - 1, vertex.coords.1);
        let mut last_directions = vertex.last_directions.clone();
        last_directions.push(Direction::Left);
        if last_directions.len() > 10 {
            last_directions.remove(0);
        }
        next_vertices.push((
            Vertex::new(coords, last_directions),
            *weights.get(&coords).unwrap(),
        ));
    }
    if vertex.coords.0 < column_count - 1
        && permitted_next_directions(&vertex.last_directions).contains(&Direction::Right)
    {
        let coords = (vertex.coords.0 + 1, vertex.coords.1);
        let mut last_directions = vertex.last_directions.clone();
        last_directions.push(Direction::Right);
        if last_directions.len() > 10 {
            last_directions.remove(0);
        }
        next_vertices.push((
            Vertex::new(coords, last_directions),
            *weights.get(&coords).unwrap(),
        ));
    }
    if vertex.coords.1 > 0
        && permitted_next_directions(&vertex.last_directions).contains(&Direction::Up)
    {
        let coords = (vertex.coords.0, vertex.coords.1 - 1);
        let mut last_directions = vertex.last_directions.clone();
        last_directions.push(Direction::Up);
        if last_directions.len() > 10 {
            last_directions.remove(0);
        }
        next_vertices.push((
            Vertex::new(coords, last_directions),
            *weights.get(&coords).unwrap(),
        ));
    }
    if vertex.coords.1 < row_count - 1
        && permitted_next_directions(&vertex.last_directions).contains(&Direction::Down)
    {
        let coords = (vertex.coords.0, vertex.coords.1 + 1);
        let mut last_directions = vertex.last_directions.clone();
        last_directions.push(Direction::Down);
        if last_directions.len() > 10 {
            last_directions.remove(0);
        }
        next_vertices.push((
            Vertex::new(coords, last_directions),
            *weights.get(&coords).unwrap(),
        ));
    }

    next_vertices
}

fn permitted_next_directions(last_directions: &[Direction]) -> Vec<Direction> {
    if last_directions.is_empty() {
        return vec![Direction::Right, Direction::Down];
    }

    let length = last_directions.len();
    let prev_direction = last_directions[length - 1];

    if length < 4
        || last_directions[length - 4..]
            .iter()
            .any(|d| *d != prev_direction)
    {
        return vec![prev_direction];
    }

    let mut allow_up = true;
    let mut allow_right = true;
    let mut allow_down = true;
    let mut allow_left = true;

    // Prevent immediate backtracking.
    match prev_direction {
        Direction::Up => allow_down = false,
        Direction::Right => allow_left = false,
        Direction::Down => allow_up = false,
        Direction::Left => allow_right = false,
    };

    if length >= 10
        && last_directions[length - 10..]
            .iter()
            .all(|d| *d == prev_direction)
    {
        match prev_direction {
            Direction::Up => allow_up = false,
            Direction::Right => allow_right = false,
            Direction::Down => allow_down = false,
            Direction::Left => allow_left = false,
        };
    }

    let mut new_directions = vec![];
    if allow_up {
        new_directions.push(Direction::Up);
    }
    if allow_right {
        new_directions.push(Direction::Right);
    }
    if allow_down {
        new_directions.push(Direction::Down);
    }
    if allow_left {
        new_directions.push(Direction::Left);
    }

    new_directions
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Vertex {
    coords: Coords,
    last_directions: Vec<Direction>,
}

impl Vertex {
    fn new(coords: Coords, last_directions: Vec<Direction>) -> Self {
        Self {
            coords,
            last_directions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
        ";

        assert_eq!(run(input), 94);
    }
}
