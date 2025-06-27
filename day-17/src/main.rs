use std::str::FromStr;

use anyhow::{Result, anyhow};
use pathfinding::prelude::dijkstra;

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-17.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<u32> {
    let grid = Grid::from_str(input)?;

    dijkstra(
        &((0, 0), Default::default()),
        |node| successors(node, &grid),
        |node| success(node, &grid),
    )
    .map(|shortest_path| shortest_path.1)
    .ok_or(anyhow!("Cannot find shortest path"))
}

fn part_2(input: &str) -> Result<u32> {
    let grid = Grid::from_str(input)?;

    dijkstra(
        &(
            (0, 0),
            Crucible {
                crucible_type: CrucibleType::Ultra,
                ..Default::default()
            },
        ),
        |node| successors(node, &grid),
        |node| success(node, &grid),
    )
    .map(|shortest_path| shortest_path.1)
    .ok_or(anyhow!("Cannot find shortest path"))
}

struct Grid {
    heat_loss: Vec<Vec<u8>>,
    size: usize,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        let size = lines.len();
        let heat_loss = lines
            .into_iter()
            .map(|line| {
                line.bytes()
                    .map(|byte| {
                        if byte.is_ascii_digit() {
                            Ok(byte - b'0')
                        } else {
                            Err(anyhow!("Invalid byte: {}", byte))
                        }
                    })
                    .collect()
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { heat_loss, size })
    }
}

/// (row, col)
type Coord = (usize, usize);

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    None,
}

type Node = (Coord, Crucible);

fn successors(node: &Node, grid: &Grid) -> Vec<(Node, u32)> {
    let &((row, col), crucible) = node;

    let mut successors = Vec::new();
    if row > 0 && crucible.can_proceed_in_direction(Direction::Up) {
        successors.push((
            ((row - 1, col), crucible.proceed_in_direction(Direction::Up)),
            grid.heat_loss[row - 1][col] as u32,
        ));
    }
    if col < grid.size - 1 && crucible.can_proceed_in_direction(Direction::Right) {
        successors.push((
            (
                (row, col + 1),
                crucible.proceed_in_direction(Direction::Right),
            ),
            grid.heat_loss[row][col + 1] as u32,
        ));
    }
    if row < grid.size - 1 && crucible.can_proceed_in_direction(Direction::Down) {
        successors.push((
            (
                (row + 1, col),
                crucible.proceed_in_direction(Direction::Down),
            ),
            grid.heat_loss[row + 1][col] as u32,
        ));
    }
    if col > 0 && crucible.can_proceed_in_direction(Direction::Left) {
        successors.push((
            (
                (row, col - 1),
                crucible.proceed_in_direction(Direction::Left),
            ),
            grid.heat_loss[row][col - 1] as u32,
        ));
    }

    successors
}

fn success(node: &Node, grid: &Grid) -> bool {
    let &((row, col), crucible) = node;

    row == grid.size - 1 && col == grid.size - 1 && crucible.can_stop()
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
enum CrucibleType {
    #[default]
    Normal,
    Ultra,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Crucible {
    crucible_type: CrucibleType,
    direction: Direction,
    count: usize,
}

impl Default for Crucible {
    fn default() -> Self {
        Self {
            crucible_type: Default::default(),
            direction: Direction::None,
            count: 1,
        }
    }
}

impl Crucible {
    fn can_proceed_in_direction(&self, direction: Direction) -> bool {
        match (self.crucible_type, self.direction, direction) {
            (_, _, Direction::None) => false,
            (_, Direction::None, _) => true,
            (_, Direction::Up, Direction::Down)
            | (_, Direction::Right, Direction::Left)
            | (_, Direction::Down, Direction::Up)
            | (_, Direction::Left, Direction::Right) => false,
            (CrucibleType::Normal, a, b) if a == b => self.count < 3,
            (CrucibleType::Normal, _, _) => true,
            (CrucibleType::Ultra, a, b) if a == b => self.count < 10,
            (CrucibleType::Ultra, a, b) if a != b => self.count >= 4,
            _ => false,
        }
    }

    fn proceed_in_direction(&self, direction: Direction) -> Self {
        Self {
            crucible_type: self.crucible_type,
            direction,
            count: if self.direction == direction {
                self.count + 1
            } else {
                1
            },
        }
    }

    fn can_stop(&self) -> bool {
        match self.crucible_type {
            CrucibleType::Normal => true,
            CrucibleType::Ultra => self.count >= 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
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

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 102);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 94);

        Ok(())
    }
}
