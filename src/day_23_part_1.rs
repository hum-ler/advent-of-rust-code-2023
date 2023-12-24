// Brute-force the k-shortest paths until we reach max k. We can roughly estimate k by the number of
// forks in the map.

use pathfinding::directed::yen::yen;

use crate::clean_lines;

pub fn run(input: &str) -> usize {
    let tiles = clean_lines(input)
        .map(str::chars)
        .map(|c| c.collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let paths = yen(
        &((1, 0), Direction::Down),
        |s| successors(*s, &tiles),
        |s| s.0 == (tiles[0].len() - 2, tiles.len() - 1),
        300,
    );

    paths.iter().map(|p| p.1).max().unwrap()
}

fn successors(step: Step, tiles: &[Vec<char>]) -> Vec<(Step, usize)> {
    let mut links = vec![];

    match step {
        ((x, y), Direction::Up) => {
            match tiles[y][x - 1] {
                '.' => links.push((((x - 1, y), Direction::Left), 1)),
                'v' => links.push((((x - 1, y + 1), Direction::Down), 2)),
                _ => (),
            };

            match tiles[y - 1][x] {
                '.' => links.push((((x, y - 1), Direction::Up), 1)),
                '>' => links.push((((x + 1, y - 1), Direction::Right), 2)),
                _ => (),
            };

            match tiles[y][x + 1] {
                '.' => links.push((((x + 1, y), Direction::Right), 1)),
                '>' => links.push((((x + 2, y), Direction::Right), 2)),
                'v' => links.push((((x + 1, y + 1), Direction::Down), 2)),
                _ => (),
            };
        }
        ((x, y), Direction::Right) => {
            match tiles[y - 1][x] {
                '.' => links.push((((x, y - 1), Direction::Up), 1)),
                '>' => links.push((((x + 1, y - 1), Direction::Right), 2)),
                _ => (),
            };
            match tiles[y][x + 1] {
                '.' => links.push((((x + 1, y), Direction::Right), 1)),
                '>' => links.push((((x + 2, y), Direction::Right), 2)),
                'v' => links.push((((x + 1, y + 1), Direction::Down), 2)),
                _ => (),
            };
            match tiles[y + 1][x] {
                '.' => links.push((((x, y + 1), Direction::Down), 1)),
                '>' => links.push((((x + 1, y + 1), Direction::Right), 2)),
                'v' => links.push((((x, y + 2), Direction::Down), 2)),
                _ => (),
            };
        }
        ((x, y), Direction::Down) => {
            match tiles[y][x + 1] {
                '.' => links.push((((x + 1, y), Direction::Right), 1)),
                '>' => links.push((((x + 2, y), Direction::Right), 2)),
                'v' => links.push((((x + 1, y + 1), Direction::Down), 2)),
                _ => (),
            };
            match tiles[y + 1][x] {
                '.' => links.push((((x, y + 1), Direction::Down), 1)),
                '>' => links.push((((x + 1, y + 1), Direction::Right), 2)),
                'v' => links.push((((x, y + 2), Direction::Down), 2)),
                _ => (),
            };
            match tiles[y][x - 1] {
                '.' => links.push((((x - 1, y), Direction::Left), 1)),
                'v' => links.push((((x - 1, y + 1), Direction::Down), 2)),
                _ => (),
            };
        }
        ((x, y), Direction::Left) => {
            match tiles[y + 1][x] {
                '.' => links.push((((x, y + 1), Direction::Down), 1)),
                '>' => links.push((((x + 1, y + 1), Direction::Right), 2)),
                'v' => links.push((((x, y + 2), Direction::Down), 2)),
                _ => (),
            };
            match tiles[y][x - 1] {
                '.' => links.push((((x - 1, y), Direction::Left), 1)),
                'v' => links.push((((x - 1, y + 1), Direction::Down), 2)),
                _ => (),
            };
            match tiles[y - 1][x] {
                '.' => links.push((((x, y - 1), Direction::Up), 1)),
                '>' => links.push((((x + 1, y - 1), Direction::Right), 2)),
                _ => (),
            };
        }
    }

    links
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub(crate) type Coords = (usize, usize);

type Step = (Coords, Direction);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
            #.#####################
            #.......#########...###
            #######.#########.#.###
            ###.....#.>.>.###.#.###
            ###v#####.#v#.###.#.###
            ###.>...#.#.#.....#...#
            ###v###.#.#.#########.#
            ###...#.#.#.......#...#
            #####.#.#.#######.#.###
            #.....#.#.#.......#...#
            #.#####.#.#.#########v#
            #.#...#...#...###...>.#
            #.#.#v#######v###.###v#
            #...#.>.#...>.>.#.###.#
            #####v#.#.###v#.#.###.#
            #.....#...#...#.#.#...#
            #.#########.###.#.#.###
            #...###...#...#...#.###
            ###.###.#.###v#####v###
            #...#...#.#.>.>.#.>.###
            #.###.###.#.###.#.#v###
            #.....###...###...#...#
            #####################.#
        ";

        assert_eq!(run(input), 94);
    }
}
