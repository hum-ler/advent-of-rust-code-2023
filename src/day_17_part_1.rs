use std::{collections::HashMap, hash::Hash};

use pathfinding::prelude::dijkstra;

use crate::clean_lines;

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
        &Vertex::new((0, 0), [None, None, None]),
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
        && !vertex
            .last_directions
            .iter()
            .all(|d| *d == Some(Direction::Left))
        && vertex.last_directions[2] != Some(Direction::Right)
    {
        let next_coords = (vertex.coords.0 - 1, vertex.coords.1);
        next_vertices.push((
            Vertex::new(
                next_coords,
                [
                    vertex.last_directions[1],
                    vertex.last_directions[2],
                    Some(Direction::Left),
                ],
            ),
            *weights.get(&next_coords).unwrap(),
        ));
    }
    if vertex.coords.0 < column_count - 1
        && !vertex
            .last_directions
            .iter()
            .all(|d| *d == Some(Direction::Right))
        && vertex.last_directions[2] != Some(Direction::Left)
    {
        let next_coords = (vertex.coords.0 + 1, vertex.coords.1);
        next_vertices.push((
            Vertex::new(
                next_coords,
                [
                    vertex.last_directions[1],
                    vertex.last_directions[2],
                    Some(Direction::Right),
                ],
            ),
            *weights.get(&next_coords).unwrap(),
        ));
    }
    if vertex.coords.1 > 0
        && !vertex
            .last_directions
            .iter()
            .all(|d| *d == Some(Direction::Up))
        && vertex.last_directions[2] != Some(Direction::Down)
    {
        let next_coords = (vertex.coords.0, vertex.coords.1 - 1);
        next_vertices.push((
            Vertex::new(
                next_coords,
                [
                    vertex.last_directions[1],
                    vertex.last_directions[2],
                    Some(Direction::Up),
                ],
            ),
            *weights.get(&next_coords).unwrap(),
        ));
    }
    if vertex.coords.1 < row_count - 1
        && !vertex
            .last_directions
            .iter()
            .all(|d| *d == Some(Direction::Down))
        && vertex.last_directions[2] != Some(Direction::Up)
    {
        let next_coords = (vertex.coords.0, vertex.coords.1 + 1);
        next_vertices.push((
            Vertex::new(
                next_coords,
                [
                    vertex.last_directions[1],
                    vertex.last_directions[2],
                    Some(Direction::Down),
                ],
            ),
            *weights.get(&next_coords).unwrap(),
        ));
    }

    next_vertices
}

pub(crate) type Coords = (usize, usize);

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(crate) enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Vertex {
    coords: Coords,
    last_directions: [Option<Direction>; 3],
}

impl Vertex {
    fn new(coords: Coords, path: [Option<Direction>; 3]) -> Self {
        Self {
            coords,
            last_directions: path,
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

        assert_eq!(run(input), 102);
    }
}
