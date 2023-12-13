// The strategy:
// 1. Traverse the main loop once, mark down all main loop pipes.
// 2. Traverse the main loop a second time in clockwise direction. At each step, we look to our rhs,
// and mark all tiles as enclosed until we hit another main loop pipe.
// 3. Count all the enclosed tiles.
//
// Rhs means:
// 1. If we traverse north, we check the tiles to the east.
// 2. If we traverse east, we check the south.
// 3. If we traverse south, we check the west.
// 4. If we traverse west, we check the north.
//
// To determine which direction is considered clockwise, consider the row containing S and look for
// the left-most main loop pipe. This pipe can only be '|', 'L' or 'F'. We always move towards north
// or east first.

use std::collections::HashMap;

use crate::clean_lines;

pub fn run(input: &str) -> usize {
    let (start, mut hash_map) = parse_map(input);

    let mut main_loop = get_main_loop(start, &mut hash_map);

    if should_reverse_loop(start, &main_loop, &mut hash_map) {
        main_loop = reverse_main_loop(&main_loop);
    }

    let directions = get_directions(&main_loop);

    // Mark all according to the enter and exit directions.
    for (index, coords) in main_loop.iter().enumerate() {
        let (enter, exit) = directions[index];

        match enter {
            Direction::North => mark_enclosed(*coords, Direction::East, &mut hash_map, &main_loop),
            Direction::East => mark_enclosed(*coords, Direction::South, &mut hash_map, &main_loop),
            Direction::South => mark_enclosed(*coords, Direction::West, &mut hash_map, &main_loop),
            Direction::West => mark_enclosed(*coords, Direction::North, &mut hash_map, &main_loop),
        }

        if enter == exit {
            // straight pipe
            continue;
        }

        match exit {
            Direction::North => mark_enclosed(*coords, Direction::East, &mut hash_map, &main_loop),
            Direction::East => mark_enclosed(*coords, Direction::South, &mut hash_map, &main_loop),
            Direction::South => mark_enclosed(*coords, Direction::West, &mut hash_map, &main_loop),
            Direction::West => mark_enclosed(*coords, Direction::North, &mut hash_map, &main_loop),
        }
    }

    hash_map
        .values()
        .filter(|tile| **tile == Tile::Enclosed)
        .count()
}

fn get_main_loop(start: Coords, hash_map: &mut HashMap<Coords, Tile>) -> Vec<Coords> {
    let mut main_loop = vec![start];

    let mut from = start;
    let mut current = get_start_exit(start, hash_map);
    loop {
        if current == start {
            break;
        }

        main_loop.push(current);

        let next = traverse(current, from, hash_map);
        from = current;
        current = next;
    }

    main_loop
}

fn reverse_main_loop(main_loop: &[Coords]) -> Vec<Coords> {
    let mut reverse = Vec::from(main_loop);
    reverse.reverse();
    let start = reverse.pop().unwrap();
    reverse.insert(0, start);

    reverse
}

fn should_reverse_loop(
    start: Coords,
    main_loop: &Vec<Coords>,
    hash_map: &mut HashMap<Coords, Tile>,
) -> bool {
    let row = start.1;

    // Find the first main loop pipe.
    let mut first_pipe = (0, 0);
    for column in 0..=start.0 {
        if !main_loop.contains(&(column, row)) {
            continue;
        }

        first_pipe = (column, row);
        break;
    }

    // Find the next main loop pipe.
    let first_index = main_loop
        .iter()
        .position(|pipe| pipe == &first_pipe)
        .unwrap();
    let next_index = if first_index + 1 == main_loop.len() {
        0
    } else {
        first_index + 1
    };
    let next_pipe = main_loop[next_index];

    // Check if next main loop pipe is the expected one based on the tile type.
    match hash_map.get(&first_pipe).unwrap() {
        Tile::NorthSouthPipe => next_pipe != north(first_pipe),
        Tile::NorthEastPipe => next_pipe != north(first_pipe),
        Tile::SouthEastPipe => next_pipe != east(first_pipe),
        Tile::Start => false, // always allocated correctly in initial mapout
        other => panic!("Unexpected tile {other:?} when checking for reversing"),
    }
}

/// Deduces the enter and exit directions for each pipe inside main_loop.
/// Returns the list of (enter_direction, exit_direction) tuples.
fn get_directions(main_loop: &Vec<Coords>) -> Vec<(Direction, Direction)> {
    let mut enter_directions = main_loop
        .windows(2)
        .map(|window| get_direction(window[0], window[1]))
        .collect::<Vec<Direction>>();
    enter_directions.insert(
        0,
        get_direction(main_loop[main_loop.len() - 1], main_loop[0]),
    );

    let mut exit_directions = enter_directions.clone();
    let head = exit_directions.remove(0);
    exit_directions.push(head);

    enter_directions
        .into_iter()
        .zip(exit_directions)
        .collect::<Vec<(Direction, Direction)>>()
}

fn get_direction(from: Coords, to: Coords) -> Direction {
    // Need to guard the northern and western boundaries.
    if from.1 > 0 && north(from) == to {
        Direction::North
    } else if east(from) == to {
        Direction::East
    } else if south(from) == to {
        Direction::South
    } else {
        Direction::West
    }
}

/// Converts all non-main-loop tiles to Tile::Enclosed in the given direction.
///
/// Starts from coords, keep moving along the direction until we hit another main loop pipe, convert
/// every tile we step on into enclosed as we move.
fn mark_enclosed(
    coords: Coords,
    direction: Direction,
    hash_map: &mut HashMap<Coords, Tile>,
    main_loop: &[Coords],
) {
    match direction {
        Direction::North => {
            let mut target_coords = (coords.0, coords.1 - 1);
            loop {
                if main_loop.contains(&target_coords) {
                    return;
                }

                hash_map.insert(target_coords, Tile::Enclosed);

                target_coords = (target_coords.0, target_coords.1 - 1);
            }
        }
        Direction::East => {
            let mut target_coords = (coords.0 + 1, coords.1);
            loop {
                if main_loop.contains(&target_coords) {
                    return;
                }

                hash_map.insert(target_coords, Tile::Enclosed);

                target_coords = (target_coords.0 + 1, target_coords.1);
            }
        }
        Direction::South => {
            let mut target_coords = (coords.0, coords.1 + 1);
            loop {
                if main_loop.contains(&target_coords) {
                    return;
                }

                hash_map.insert(target_coords, Tile::Enclosed);

                target_coords = (target_coords.0, target_coords.1 + 1);
            }
        }
        Direction::West => {
            let mut target_coords = (coords.0 - 1, coords.1);
            loop {
                if main_loop.contains(&target_coords) {
                    return;
                }

                hash_map.insert(target_coords, Tile::Enclosed);

                target_coords = (target_coords.0 - 1, target_coords.1);
            }
        }
    }
}

type Coords = (usize, usize);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq)]
enum Tile {
    NorthSouthPipe,
    EastWestPipe,
    NorthEastPipe,
    NorthWestPipe,
    SouthWestPipe,
    SouthEastPipe,
    Ground,
    Start,
    Enclosed,
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
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        ";
        assert_eq!(run(input), 4);

        let input = r"
            ..........
            .S------7.
            .|F----7|.
            .||....||.
            .||....||.
            .|L-7F-J|.
            .|..||..|.
            .L--JL--J.
            ..........
        ";
        assert_eq!(run(input), 4);

        let input = r"
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        ";
        assert_eq!(run(input), 8);

        let input = r"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        ";
        assert_eq!(run(input), 10);
    }
}
