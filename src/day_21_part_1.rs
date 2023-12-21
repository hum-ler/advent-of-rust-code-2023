use std::collections::{HashMap, HashSet};

use crate::clean_lines;

pub fn run(input: &str) -> usize {
    run_steps(64, input)
}

fn run_steps(steps: usize, input: &str) -> usize {
    let (hash_map, starting_position, map_size) = parse_map(input);

    let mut positions = HashSet::new();
    positions.insert(starting_position);

    for _ in 0..steps {
        let mut positions_to_check = Vec::from_iter(positions.drain());

        while let Some(position) = positions_to_check.pop() {
            neighbors_4(position, map_size)
                .iter()
                .for_each(|next_position| {
                    if let Some(Tile::Ground) = hash_map.get(next_position) {
                        positions.insert(*next_position);
                    }
                });
        }
    }

    // print_map(&positions, &hash_map, map_size);
    // println!();

    positions.len()
}

// fn print_map(positions: &HashSet<Coords>, hash_map: &HashMap<Coords, Tile>, map_size: Coords) {
//     (0..map_size.1).for_each(|y| {
//         (0..map_size.0).for_each(|x| {
//             if positions.contains(&(x, y)) {
//                 print!("O");
//             } else if hash_map.get(&(x, y)).unwrap() == &Tile::Rock {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         });
//         println!();
//     });
// }

fn neighbors_4(coords: Coords, map_size: Coords) -> Vec<Coords> {
    let mut neighbors = vec![];

    if coords.1 > 0 {
        neighbors.push((coords.0, coords.1 - 1));
    }
    if coords.0 < map_size.0 - 1 {
        neighbors.push((coords.0 + 1, coords.1));
    }
    if coords.1 < map_size.1 - 1 {
        neighbors.push((coords.0, coords.1 + 1));
    }
    if coords.0 > 0 {
        neighbors.push((coords.0 - 1, coords.1));
    }

    neighbors
}

/// Parses the input.
///
/// Returns a map of Coords to Tile, the starting position, and the map size.
fn parse_map(input: &str) -> (HashMap<Coords, Tile>, Coords, Coords) {
    let mut hash_map = HashMap::new();
    let mut starting_position = (0, 0);

    let lines = clean_lines(input).collect::<Vec<&str>>();
    lines.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
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
        })
    });

    (hash_map, starting_position, (lines[0].len(), lines.len()))
}

type Coords = (usize, usize);

#[derive(PartialEq)]
enum Tile {
    Ground,
    Rock,
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(run_steps(6, input), 16);
    }
}
