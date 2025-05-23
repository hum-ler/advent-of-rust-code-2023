use std::{collections::HashSet, str::FromStr};

use anyhow::Result;
use itertools::Itertools;

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-11.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<usize> {
    distances_after_expansion(input, 2)
}

fn part_2(input: &str) -> Result<usize> {
    distances_after_expansion(input, 1000000)
}

fn distances_after_expansion(input: &str, replace_size: usize) -> Result<usize> {
    let mut grid = Grid::from_str(input)?;
    grid.expand(replace_size);

    Ok(grid
        .galaxies
        .iter()
        .tuple_combinations()
        .map(|(coord_1, coord_2)| manhatten_distance(*coord_1, *coord_2))
        .sum())
}

/// (row, col)
type Coord = (usize, usize);

struct Grid {
    galaxies: HashSet<Coord>,
    size: usize,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        let size = lines.len();
        let galaxies = lines
            .into_iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.bytes().enumerate().filter_map(move |(col, byte)| {
                    if byte == b'#' { Some((row, col)) } else { None }
                })
            })
            .collect::<HashSet<_>>();

        Ok(Self { galaxies, size })
    }
}

impl Grid {
    /// Expands this [Grid] by replacing every empty row by replace_size rows, and every empty col
    /// by replace_size cols.
    ///
    /// replace_size must be >= 1. replace_size of 1 implies no expansion, as each empty row or
    /// col is simply replaced by itself.
    fn expand(&mut self, replace_size: usize) {
        if replace_size <= 1 {
            return;
        }

        // Mark pos of new rows and cols.
        let mut empty_rows = vec![0; self.size];
        let mut empty_cols = vec![0; self.size];
        empty_rows.iter_mut().enumerate().for_each(|(index, row)| {
            if !self.galaxies.iter().any(|galaxy| galaxy.0 == index) {
                *row = replace_size - 1;
            }
        });
        empty_cols.iter_mut().enumerate().for_each(|(index, col)| {
            if !self.galaxies.iter().any(|galaxy| galaxy.1 == index) {
                *col = replace_size - 1;
            }
        });

        // Calculate accumulative count of new rows and cols.
        empty_rows = empty_rows
            .into_iter()
            .scan(0, |state, row| {
                *state += row;
                Some(*state)
            })
            .collect();
        empty_cols = empty_cols
            .into_iter()
            .scan(0, |state, col| {
                *state += col;
                Some(*state)
            })
            .collect();

        self.galaxies = self
            .galaxies
            .iter()
            .map(|galaxy| {
                (
                    galaxy.0 + empty_rows[galaxy.0],
                    galaxy.1 + empty_cols[galaxy.1],
                )
            })
            .collect();
        self.size += empty_rows.last().unwrap_or(&0);
    }
}

fn manhatten_distance(coord: Coord, other: Coord) -> usize {
    coord.0.abs_diff(other.0) + coord.1.abs_diff(other.1)
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 374);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(
            distances_after_expansion(trim_newlines(EXAMPLE), 100)?,
            8410
        );

        Ok(())
    }
}
