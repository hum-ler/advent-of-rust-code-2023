use std::collections::{HashMap, HashSet};

use crate::clean_lines;

pub fn run(input: &str) -> usize {
    run_steps(26501365, input)
}

fn run_steps(steps: usize, input: &str) -> usize {
    let (hash_map, starting_position, map_size) = parse_map(input);

    let mut positions = HashSet::new();
    positions.insert(starting_position);

    for _ in 0..steps {
        let mut positions_to_check = Vec::from_iter(positions.drain());

        while let Some(position) = positions_to_check.pop() {
            [
                (position.0, position.1 - 1),
                (position.0 + 1, position.1),
                (position.0, position.1 + 1),
                (position.0 - 1, position.1),
            ]
            .iter()
            .for_each(|next_position| {
                if let Some(&Tile::Ground) = hash_map_get(&hash_map, *next_position, map_size) {
                    positions.insert(*next_position);
                }
            });
        }
    }

    // print_map(&positions, &hash_map, map_size);
    // println!();

    positions.len()
}

fn hash_map_get(hash_map: &HashMap<Coords, Tile>, coords: Coords, map_size: Coords) -> Option<&Tile> {
    if coords.0 < 0 || coords.0 > 10 || coords.1 < 0 || coords.1 > 10 {
        print!("({}, {}) => ", coords.0, coords.1);
    }

    let mut x = coords.0 % map_size.0;
    let mut y = coords.1 % map_size.1;

    if x < 0 {
        x = map_size.0 - x.abs();
    }
    if y < 0 {
        y = map_size.1 - y.abs();
    }

    println!("({}, {})", x, y);

    hash_map.get(&(x, y))
}

/// Parses the input.
///
/// Returns a map of Coords to Tile, the starting position, and the map size.
fn parse_map(input: &str) -> (HashMap<Coords, Tile>, Coords, Coords) {
    let mut hash_map = HashMap::new();
    let mut starting_position = (0, 0);

    let lines = clean_lines(input).collect::<Vec<&str>>();
    lines.iter().enumerate().for_each(|(y, line)| {
        let y = y.try_into().unwrap();

        line.chars().enumerate().for_each(|(x, c)| {
            let x = x.try_into().unwrap();

            match c {
                'S' => {
                    hash_map.insert((x, y), Tile::Ground);
                    starting_position = (x, y);
                }
                '.' => {
                    hash_map.insert((x, y), Tile::Ground);
                }
                '#' => {
                    hash_map.insert((x, y), Tile::Rock);
                }
                _ => unreachable!(),
            }
        })
    });

    (
        hash_map,
        starting_position,
        (
            lines[0].len().try_into().unwrap(),
            lines.len().try_into().unwrap(),
        ),
    )
}

type Coords = (i64, i64);

#[derive(PartialEq)]
enum Tile {
    Ground,
    Rock,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn run_example() {
        let input = r"
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........
        ";

        assert_eq!(run_steps(5000, input), 16733044);
    }
}
