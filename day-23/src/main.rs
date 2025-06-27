use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-23.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<usize> {
    let grid = Grid::from_str(input)?;

    Ok(longest_path_downslope(&grid))
}

fn part_2(input: &str) -> Result<usize> {
    let grid = Grid::from_str(input)?;
    let grid = grid.erase_arrows();

    let size = grid.size;
    let nodes = grid.into_nodes()?;

    longest_path((0, 1), (size - 1, size - 2), 0, HashSet::new(), &nodes)
        .ok_or(anyhow!("Cannot find longest path"))
}

struct Grid {
    layout: Vec<Vec<u8>>,
    size: usize,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        let size = lines.len();
        let layout = lines
            .into_iter()
            .map(|line| line.bytes().collect())
            .collect();

        Ok(Self { layout, size })
    }
}

impl Grid {
    fn erase_arrows(self) -> Self {
        let layout = self
            .layout
            .into_iter()
            .map(|bytes| {
                bytes
                    .into_iter()
                    .map(|byte| {
                        if matches!(byte, b'^' | b'>' | b'v' | b'<') {
                            b'.'
                        } else {
                            byte
                        }
                    })
                    .collect()
            })
            .collect();

        Self { layout, ..self }
    }

    fn into_nodes(self) -> Result<Nodes> {
        Ok(cache_to_nodes(grid_to_cache(&self)?))
    }
}

/// (row, col)
type Coord = (usize, usize);

fn longest_path_downslope(grid: &Grid) -> usize {
    find_longest_path_downslope((0, 1), Vec::new(), grid)
}

fn find_longest_path_downslope(coord: Coord, path: Vec<Coord>, grid: &Grid) -> usize {
    if coord == (grid.size - 1, grid.size - 2) {
        return path.len();
    }

    let (row, col) = coord;
    let mut paths = Vec::new();
    if row > 0 && !path.contains(&(row - 1, col)) {
        match grid.layout[row - 1][col] {
            b'.' => {
                let mut path = path.clone();
                path.push((row - 1, col));
                paths.push(path);
            }
            b'^' => {
                let mut path = path.clone();
                path.push((row - 1, col));
                path.push((row - 2, col));
                paths.push(path);
            }
            _ => (),
        }
    }
    if !path.contains(&(row, col + 1)) {
        match grid.layout[row][col + 1] {
            b'.' => {
                let mut path = path.clone();
                path.push((row, col + 1));
                paths.push(path);
            }
            b'>' => {
                let mut path = path.clone();
                path.push((row, col + 1));
                path.push((row, col + 2));
                paths.push(path);
            }
            _ => (),
        }
    }
    if !path.contains(&(row + 1, col)) {
        match grid.layout[row + 1][col] {
            b'.' => {
                let mut path = path.clone();
                path.push((row + 1, col));
                paths.push(path);
            }
            b'v' => {
                let mut path = path.clone();
                path.push((row + 1, col));
                path.push((row + 2, col));
                paths.push(path);
            }
            _ => (),
        }
    }
    if !path.contains(&(row, col - 1)) {
        match grid.layout[row][col - 1] {
            b'.' => {
                let mut path = path.clone();
                path.push((row, col - 1));
                paths.push(path);
            }
            b'<' => {
                let mut path = path.clone();
                path.push((row, col - 1));
                path.push((row, col - 2));
                paths.push(path);
            }
            _ => (),
        }
    }

    paths
        .into_iter()
        .map(|path| find_longest_path_downslope(path[path.len() - 1], path, grid))
        .max()
        .unwrap_or(0)
}

/// A junction, or the Start, or the Goal.
type Node = Coord;

/// (node, path_len)
type Connection = (Node, usize);

/// node => [connection]
type Nodes = HashMap<Node, Vec<Connection>>;

/// node => exit => connection
type Cache = HashMap<Coord, HashMap<Coord, Option<Connection>>>;

fn grid_to_cache(grid: &Grid) -> Result<Cache> {
    // Create an empty cache to fill up.
    let mut cache: Cache = HashMap::new();
    cache.entry((0, 1)).or_default().entry((1, 1)).or_default();
    cache
        .entry((grid.size - 1, grid.size - 2))
        .or_default()
        .entry((grid.size - 2, grid.size - 2))
        .or_default();

    for row in 1..grid.size - 1 {
        for col in 1..grid.size - 1 {
            if grid.layout[row][col] == b'#' {
                continue;
            }

            match (
                grid.layout[row - 1][col],
                grid.layout[row][col + 1],
                grid.layout[row + 1][col],
                grid.layout[row][col - 1],
            ) {
                (b'#', b'.', b'.', b'.') => {
                    let entry = cache.entry((row, col)).or_default();
                    entry.entry((row, col + 1)).or_default();
                    entry.entry((row + 1, col)).or_default();
                    entry.entry((row, col - 1)).or_default();
                }
                (b'.', b'#', b'.', b'.') => {
                    let entry = cache.entry((row, col)).or_default();
                    entry.entry((row - 1, col)).or_default();
                    entry.entry((row + 1, col)).or_default();
                    entry.entry((row, col - 1)).or_default();
                }
                (b'.', b'.', b'#', b'.') => {
                    let entry = cache.entry((row, col)).or_default();
                    entry.entry((row - 1, col)).or_default();
                    entry.entry((row, col + 1)).or_default();
                    entry.entry((row, col - 1)).or_default();
                }
                (b'.', b'.', b'.', b'#') => {
                    let entry = cache.entry((row, col)).or_default();
                    entry.entry((row - 1, col)).or_default();
                    entry.entry((row, col + 1)).or_default();
                    entry.entry((row + 1, col)).or_default();
                }
                (b'.', b'.', b'.', b'.') => {
                    let entry = cache.entry((row, col)).or_default();
                    entry.entry((row - 1, col)).or_default();
                    entry.entry((row, col + 1)).or_default();
                    entry.entry((row + 1, col)).or_default();
                    entry.entry((row, col - 1)).or_default();
                }
                _ => (),
            }
        }
    }

    fill_cache(&mut cache, grid)?;

    Ok(cache)
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

/// Fills up the given [Cache] with [Connection] entries.
fn fill_cache(cache: &mut Cache, grid: &Grid) -> Result<()> {
    let snapshot = cache.clone();
    let nodes = snapshot.keys().copied().collect::<Vec<_>>();
    for (node, connections) in snapshot {
        for (exit, _) in connections {
            if cache[&node][&exit].is_none() {
                let (other_exit, connection) = find_connection(node, exit, grid, &nodes)?;
                let (other_node, path_len) = connection;
                cache
                    .entry(node)
                    .or_default()
                    .entry(exit)
                    .and_modify(|value| *value = Some(connection));
                cache
                    .entry(other_node)
                    .or_default()
                    .entry(other_exit)
                    .and_modify(|value| *value = Some((node, path_len)));
            }
        }
    }

    Ok(())
}

/// Traces a path from the current [Node] exit to the next [Node].
///
/// Returns the [Connection] and the entrance into that [Connection].
fn find_connection(
    node: Coord,
    exit: Coord,
    grid: &Grid,
    nodes: &[Coord],
) -> Result<(Coord, Connection)> {
    let mut path_len = 1;
    let mut prev_coord = node;
    let (mut row, mut col) = exit;
    let mut direction = match (row.cmp(&node.0), col.cmp(&node.1)) {
        (Ordering::Less, Ordering::Equal) => Direction::Up,
        (Ordering::Equal, Ordering::Greater) => Direction::Right,
        (Ordering::Greater, Ordering::Equal) => Direction::Down,
        (Ordering::Equal, Ordering::Less) => Direction::Left,
        _ => return Err(anyhow!("Invalid node and exit: {:?}, {:?}", node, exit)),
    };
    loop {
        if nodes.contains(&(row, col)) {
            return Ok((prev_coord, ((row, col), path_len)));
        }

        prev_coord = (row, col);
        path_len += 1;

        match direction {
            Direction::Up => {
                match (
                    grid.layout[row][col - 1],
                    grid.layout[row - 1][col],
                    grid.layout[row][col + 1],
                ) {
                    (b'.', b'#', b'#') => {
                        col -= 1;
                        direction = Direction::Left;
                    }
                    (b'#', b'.', b'#') => {
                        row -= 1;
                        direction = Direction::Up;
                    }
                    (b'#', b'#', b'.') => {
                        col += 1;
                        direction = Direction::Right;
                    }
                    _ => return Err(anyhow!("Invalid path: {:?}", prev_coord)),
                }
            }
            Direction::Right => {
                match (
                    grid.layout[row - 1][col],
                    grid.layout[row][col + 1],
                    grid.layout[row + 1][col],
                ) {
                    (b'.', b'#', b'#') => {
                        row -= 1;
                        direction = Direction::Up;
                    }
                    (b'#', b'.', b'#') => {
                        col += 1;
                        direction = Direction::Right;
                    }
                    (b'#', b'#', b'.') => {
                        row += 1;
                        direction = Direction::Down;
                    }
                    _ => return Err(anyhow!("Invalid path: {:?}", prev_coord)),
                }
            }
            Direction::Down => {
                match (
                    grid.layout[row][col + 1],
                    grid.layout[row + 1][col],
                    grid.layout[row][col - 1],
                ) {
                    (b'.', b'#', b'#') => {
                        col += 1;
                        direction = Direction::Right;
                    }
                    (b'#', b'.', b'#') => {
                        row += 1;
                        direction = Direction::Down;
                    }
                    (b'#', b'#', b'.') => {
                        col -= 1;
                        direction = Direction::Left;
                    }
                    _ => return Err(anyhow!("Invalid path: {:?}", prev_coord)),
                }
            }
            Direction::Left => {
                match (
                    grid.layout[row + 1][col],
                    grid.layout[row][col - 1],
                    grid.layout[row - 1][col],
                ) {
                    (b'.', b'#', b'#') => {
                        row += 1;
                        direction = Direction::Down;
                    }
                    (b'#', b'.', b'#') => {
                        col -= 1;
                        direction = Direction::Left;
                    }
                    (b'#', b'#', b'.') => {
                        row -= 1;
                        direction = Direction::Up;
                    }
                    _ => return Err(anyhow!("Invalid path: {:?}", prev_coord)),
                }
            }
        }
    }
}

/// Flattens the given [Cache] to [Nodes].
fn cache_to_nodes(cache: Cache) -> Nodes {
    cache
        .into_iter()
        .map(|(node, connections)| (node, connections.into_values().flatten().collect()))
        .collect::<HashMap<_, _>>()
}

/// Finds the len of the longest path from node to target.
fn longest_path(
    node: Node,
    target: Node,
    path_len: usize,
    visited_nodes: HashSet<Node>,
    nodes: &Nodes,
) -> Option<usize> {
    if node == target {
        return Some(path_len);
    }

    nodes[&node]
        .iter()
        .flat_map(|(connection, connection_len)| {
            if visited_nodes.contains(connection) {
                None
            } else {
                let mut visited_nodes = visited_nodes.clone();
                visited_nodes.insert(node);
                longest_path(
                    *connection,
                    target,
                    path_len + connection_len,
                    visited_nodes,
                    nodes,
                )
            }
        })
        .max()
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
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

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 94);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 154);

        Ok(())
    }
}
