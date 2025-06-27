use std::str::FromStr;

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-13.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<usize> {
    input
        .split_terminator("\n\n")
        .map(Grid::from_str)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|grid| match grid.find_reflection(None) {
            Some(Mirror::Horizontal(row)) => Ok(100 * (row + 1)),
            Some(Mirror::Vertical(col)) => Ok(col + 1),
            _ => Err(anyhow!("Cannot find mirror: {:?}", grid.pattern)),
        })
        .sum()
}

fn part_2(input: &str) -> Result<usize> {
    input
        .split_terminator("\n\n")
        .map(Grid::from_str)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|grid| match clean_smudge_and_find_new_reflection(&grid) {
            Some(Mirror::Horizontal(row)) => Ok(100 * (row + 1)),
            Some(Mirror::Vertical(col)) => Ok(col + 1),
            _ => Err(anyhow!("Cannot find mirror: {:?}", grid.pattern)),
        })
        .sum()
}

/// (rows, cols)
type GridSize = (usize, usize);

#[derive(Clone)]
struct Grid {
    pattern: Vec<Vec<u8>>,
    size: GridSize,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        let rows = lines.len();
        let cols = lines.first().map_or(0, |line| line.len());
        let size = (rows, cols);

        let pattern = lines
            .into_iter()
            .map(|line| line.bytes().collect())
            .collect();

        Ok(Self { pattern, size })
    }
}

impl Grid {
    fn find_reflection(&self, ignore_mirror: Option<Mirror>) -> Option<Mirror> {
        self.find_horizontal_reflection(ignore_mirror)
            .or_else(|| self.find_vertical_reflection(ignore_mirror))
    }

    fn find_horizontal_reflection(&self, ignore_mirror: Option<Mirror>) -> Option<Mirror> {
        for split_after_row in 0..=self.pattern.len() - 2 {
            if let Some(Mirror::Horizontal(row)) = ignore_mirror {
                if split_after_row == row {
                    continue;
                }
            }

            if Self::split_and_compare(&self.pattern, split_after_row) {
                return Some(Mirror::Horizontal(split_after_row));
            }
        }

        None
    }

    fn find_vertical_reflection(&self, ignore_mirror: Option<Mirror>) -> Option<Mirror> {
        let transposed = self.transpose_pattern();

        for split_after_col in 0..=transposed.len() - 2 {
            if let Some(Mirror::Vertical(col)) = ignore_mirror {
                if split_after_col == col {
                    continue;
                }
            }

            if Self::split_and_compare(&transposed, split_after_col) {
                return Some(Mirror::Vertical(split_after_col));
            }
        }

        None
    }

    fn transpose_pattern(&self) -> Vec<Vec<u8>> {
        let mut transposed = vec![vec![0; self.size.0]; self.size.1];

        for (row, bytes) in self.pattern.iter().enumerate() {
            for (col, byte) in bytes.iter().enumerate() {
                transposed[col][row] = *byte;
            }
        }

        transposed
    }

    fn split_and_compare(pattern: &[Vec<u8>], split_after_row: usize) -> bool {
        let (top_half, bottom_half) = if split_after_row < pattern.len() / 2 {
            (
                &pattern[..=split_after_row],
                &pattern[split_after_row + 1..=split_after_row * 2 + 1],
            )
        } else {
            (
                &pattern[2 * split_after_row + 2 - pattern.len()..=split_after_row],
                &pattern[split_after_row + 1..],
            )
        };

        top_half
            .iter()
            .enumerate()
            .all(|(index, row)| *row == bottom_half[bottom_half.len() - 1 - index])
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Mirror {
    /// Horizontal mirror after row index.
    Horizontal(usize),

    /// Vertical mirror after col index.
    Vertical(usize),
}

fn clean_smudge_and_find_new_reflection(grid: &Grid) -> Option<Mirror> {
    let orig_mirror = grid.find_reflection(None)?;

    for row in 0..grid.size.0 {
        for col in 0..grid.size.1 {
            let mut grid = grid.clone();
            grid.pattern[row][col] = if grid.pattern[row][col] == b'#' {
                b'.'
            } else {
                b'#'
            };

            if let Some(new_mirror) = grid.find_reflection(Some(orig_mirror)) {
                return Some(new_mirror);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 405);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 400);

        Ok(())
    }
}
