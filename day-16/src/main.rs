use std::{collections::HashSet, str::FromStr};

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-16.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<usize> {
    let grid = Grid::from_str(input)?;

    Ok(grid.count_energised_tiles(((0, 0), Direction::Right)))
}

fn part_2(input: &str) -> Result<usize> {
    let grid = Grid::from_str(input)?;

    (0..grid.size)
        .flat_map(|index| {
            [
                ((0, index), Direction::Down),
                ((index, grid.size - 1), Direction::Left),
                ((grid.size - 1, index), Direction::Up),
                ((index, 0), Direction::Right),
            ]
        })
        .map(|beam| grid.count_energised_tiles(beam))
        .max()
        .ok_or(anyhow!("Cannot find max energised tiles"))
}

struct Grid {
    tiles: Vec<Vec<u8>>,
    size: usize,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        let size = lines.len();
        let tiles = lines
            .into_iter()
            .map(|line| line.bytes().collect())
            .collect();

        Ok(Self { tiles, size })
    }
}

impl Grid {
    fn traverse_beam(&self, beam: Beam) -> Vec<Beam> {
        let ((row, col), direction) = beam;

        let mut beams = Vec::new();
        match (self.tiles[row][col], direction) {
            (b'.' | b'|', Direction::Up) | (b'/', Direction::Right) | (b'\\', Direction::Left) => {
                if row > 0 {
                    beams.push(((row - 1, col), Direction::Up))
                }
            }
            (b'.' | b'-', Direction::Right) | (b'/', Direction::Up) | (b'\\', Direction::Down) => {
                if col < self.size - 1 {
                    beams.push(((row, col + 1), Direction::Right))
                }
            }
            (b'.' | b'|', Direction::Down)
            | (b'/', Direction::Left)
            | (b'\\', Direction::Right) => {
                if row < self.size - 1 {
                    beams.push(((row + 1, col), Direction::Down))
                }
            }
            (b'.' | b'-', Direction::Left) | (b'/', Direction::Down) | (b'\\', Direction::Up) => {
                if col > 0 {
                    beams.push(((row, col - 1), Direction::Left))
                }
            }
            (b'|', Direction::Right | Direction::Left) => {
                if row > 0 {
                    beams.push(((row - 1, col), Direction::Up));
                }
                if row < self.size - 1 {
                    beams.push(((row + 1, col), Direction::Down));
                }
            }
            (b'-', Direction::Up | Direction::Down) => {
                if col < self.size - 1 {
                    beams.push(((row, col + 1), Direction::Right));
                }
                if col > 0 {
                    beams.push(((row, col - 1), Direction::Left));
                }
            }
            _ => (),
        }

        beams
    }

    fn count_energised_tiles(&self, beam: Beam) -> usize {
        let mut beam_trail = HashSet::new();
        let mut beams = vec![beam];
        while let Some(beam) = beams.pop() {
            if beam_trail.insert(beam) {
                beams.extend(self.traverse_beam(beam));
            }
        }

        beam_trail
            .into_iter()
            .map(|beam| beam.0)
            .collect::<HashSet<_>>()
            .len()
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
}

type Beam = (Coord, Direction);

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 46);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 51);

        Ok(())
    }
}
