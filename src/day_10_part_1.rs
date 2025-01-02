use std::collections::HashMap;

use crate::clean_lines;

pub fn run(input: &str) -> u32 {
    let (start, mut hash_map) = parse_map(input);

    let mut from = start;
    let mut current = get_start_exit(start, &mut hash_map);
    let mut counter = 1;

    loop {
        let next = traverse(current, from, &mut hash_map);
        from = current;
        current = next;
        counter += 1;

        if current == start {
            break;
        }
    }

    counter / 2
}

type Coords = (usize, usize);

enum Tile {
    NorthSouthPipe,
    EastWestPipe,
    NorthEastPipe,
    NorthWestPipe,
    SouthWestPipe,
    SouthEastPipe,
    Ground,
    Start,
}

fn parse_map(input: &str) -> (Coords, HashMap<Coords, Tile>) {
    let mut hash_map = HashMap::new();

    let start = clean_lines(input)
        .enumerate()
        .map(|(row, token)| parse_line(token, row, &mut hash_map))
        .reduce(|acc, tile| acc.or(tile))
        .unwrap()
        .unwrap();

    (start, hash_map)
}

fn parse_line(input: &str, row: usize, hash_map: &mut HashMap<Coords, Tile>) -> Option<Coords> {
    let mut start: Option<Coords> = None;

    input.chars().enumerate().for_each(|(column, token)| {
        let coords = (column, row);

        match token {
            '|' => hash_map.insert(coords, Tile::NorthSouthPipe),
            '-' => hash_map.insert(coords, Tile::EastWestPipe),
            'L' => hash_map.insert(coords, Tile::NorthEastPipe),
            'F' => hash_map.insert(coords, Tile::SouthEastPipe),
            '7' => hash_map.insert(coords, Tile::SouthWestPipe),
            'J' => hash_map.insert(coords, Tile::NorthWestPipe),
            'S' => {
                hash_map.insert(coords, Tile::Start);

                start = Some(coords);

                None
            }
            '.' => hash_map.insert(coords, Tile::Ground),
            _ => panic!("Unexpected token {token}"),
        };
    });

    start
}

fn north(current: Coords) -> Coords {
    (current.0, current.1 - 1) // will panic if OOB
}

fn east(current: Coords) -> Coords {
    (current.0 + 1, current.1)
}

fn south(current: Coords) -> Coords {
    (current.0, current.1 + 1)
}

fn west(current: Coords) -> Coords {
    (current.0 - 1, current.1) // will panic if OOB
}

fn get_start_exit(start: Coords, hash_map: &mut HashMap<Coords, Tile>) -> Coords {
    // Need to guard the northern and western boundaries.
    if start.1 != 0 {
        let potential_exit = north(start);
        match hash_map.get(&potential_exit).unwrap() {
            Tile::NorthSouthPipe | Tile::SouthEastPipe | Tile::SouthWestPipe => {
                return potential_exit
            }
            _ => (),
        }
    }

    let potential_exit = east(start);
    match hash_map.get(&potential_exit).unwrap() {
        Tile::EastWestPipe | Tile::NorthWestPipe | Tile::SouthWestPipe => return potential_exit,
        _ => (),
    }

    south(start)
}

/// Moves from one tile to the next.
/// - current -- the current tile we are going to exit from.
/// - from -- the previous tile from which we entered the current tile.
///
/// Returns the coords of the tile we are exiting into.
///
/// from -> current -> into
fn traverse(current: Coords, from: Coords, hash_map: &mut HashMap<Coords, Tile>) -> Coords {
    match hash_map.get(&current).unwrap() {
        Tile::NorthSouthPipe => {
            if from == north(current) {
                south(current)
            } else {
                north(current)
            }
        }
        Tile::EastWestPipe => {
            if from == east(current) {
                west(current)
            } else {
                east(current)
            }
        }
        Tile::NorthEastPipe => {
            if from == north(current) {
                east(current)
            } else {
                north(current)
            }
        }
        Tile::SouthEastPipe => {
            if from == south(current) {
                east(current)
            } else {
                south(current)
            }
        }
        Tile::SouthWestPipe => {
            if from == south(current) {
                west(current)
            } else {
                south(current)
            }
        }
        Tile::NorthWestPipe => {
            if from == north(current) {
                west(current)
            } else {
                north(current)
            }
        }
        _ => panic!("Cannot traverse from {from:?} through {current:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
        ";
        assert_eq!(run(input), 4);

        let input = r"
            -L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF
        ";
        assert_eq!(run(input), 4);

        let input = r"
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
        ";
        assert_eq!(run(input), 8);

        let input = r"
            7-F7-
            .FJ|7
            SJLL7
            |F--J
            LJ.LJ
        ";
        assert_eq!(run(input), 8);
    }
}
