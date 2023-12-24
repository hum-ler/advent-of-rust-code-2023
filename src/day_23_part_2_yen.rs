// Brute-force the k-shortest paths until we reach max k. We can roughly estimate k by the number of
// forks in the map.

use pathfinding::directed::yen::yen;

use crate::{clean_lines, day_23_part_1::*};

pub fn run(input: &str) -> usize {
    let tiles = clean_lines(input)
        .map(str::chars)
        .map(|c| c.collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let paths = yen(
        &(1, 0),
        |s| successors(*s, &tiles),
        |s| *s == (tiles[0].len() - 2, tiles.len() - 1),
        100000,
    );

    println!("paths found: {}", paths.len());

    paths.iter().map(|p| p.1).max().unwrap()
}

fn successors(coords: Coords, tiles: &[Vec<char>]) -> Vec<(Coords, usize)> {
    let mut links = vec![];

    let (x, y) = coords;
    if y > 0 {
        match tiles[y - 1][x] {
            '#' => (),
            _ => links.push(((x, y - 1), 1)),
        };
    }

    match tiles[y][x + 1] {
        '#' => (),
        _ => links.push(((x + 1, y), 1)),
    };

    match tiles[y + 1][x] {
        '#' => (),
        _ => links.push(((x, y + 1), 1)),
    };

    if x > 0 {
        match tiles[y][x - 1] {
            '#' => (),
            _ => links.push(((x - 1, y), 1)),
        };
    }

    links
}

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

        assert_eq!(run(input), 154);
    }
}
