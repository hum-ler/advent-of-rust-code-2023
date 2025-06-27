use std::str::FromStr;

use anyhow::Result;

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-3.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<u32> {
    let grid = Grid::from_str(input)?;

    let mut part_numbers = (0..grid.size)
        .flat_map(|row| {
            (0..grid.size)
                .map(move |col| (row, col))
                .filter_map(|coord| get_part_number(coord, &grid))
        })
        .collect::<Vec<_>>();
    part_numbers.dedup();

    Ok(part_numbers.into_iter().sum())
}

fn part_2(input: &str) -> Result<u32> {
    let grid = Grid::from_str(input)?;

    Ok((0..grid.size)
        .flat_map(|row| {
            (0..grid.size)
                .map(move |col| (row, col))
                .filter_map(|coord| get_gear_ratio(coord, &grid))
        })
        .sum())
}

struct Grid {
    layout: Vec<Vec<u8>>,
    size: usize,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        let size = lines.first().map_or(0, |line| line.len());
        let layout = lines
            .into_iter()
            .map(|line| line.bytes().collect())
            .collect();

        Ok(Self { layout, size })
    }
}

/// (row, col)
type Coord = (usize, usize);

/// Gets all neighbouring [Coord]s starting from NW, clockwise.
fn neighbours(coord: Coord, grid: &Grid) -> Vec<Coord> {
    let (row, col) = coord;

    let mut neighbours = Vec::new();
    if row > 0 && col > 0 {
        neighbours.push((row - 1, col - 1));
    }
    if row > 0 {
        neighbours.push((row - 1, col));
    }
    if row > 0 && col < grid.size - 1 {
        neighbours.push((row - 1, col + 1));
    }
    if col < grid.size - 1 {
        neighbours.push((row, col + 1));
    }
    if row < grid.size - 1 && col < grid.size - 1 {
        neighbours.push((row + 1, col + 1));
    }
    if row < grid.size - 1 {
        neighbours.push((row + 1, col));
    }
    if row < grid.size - 1 && col > 0 {
        neighbours.push((row + 1, col - 1));
    }
    if col > 0 {
        neighbours.push((row, col - 1));
    }

    neighbours
}

/// Converts the digits between left and right [Coord]s (inclusive) into a number.
///
/// left and right must be on the same row, and left col <= right col.
fn as_number(left: Coord, right: Coord, grid: &Grid) -> u32 {
    let (row, left_col) = left;
    let (_, right_col) = right;

    let mut part_number = 0;
    for col in left_col..=right_col {
        part_number = part_number * 10 + (grid.layout[row][col] - b'0') as u32
    }

    part_number
}

fn get_part_number(coord: Coord, grid: &Grid) -> Option<u32> {
    let (row, col) = coord;

    if !grid.layout[row][col].is_ascii_digit() {
        return None;
    }

    if !neighbours(coord, grid).into_iter().any(|coord| {
        grid.layout[coord.0][coord.1] != b'.'
            && grid.layout[coord.0][coord.1].is_ascii_punctuation()
    }) {
        return None;
    }

    let mut left_col = col;
    while left_col > 0 && grid.layout[row][left_col - 1].is_ascii_digit() {
        left_col -= 1;
    }
    let mut right_col = col;
    while right_col < grid.size - 1 && grid.layout[row][right_col + 1].is_ascii_digit() {
        right_col += 1;
    }

    Some(as_number((row, left_col), (row, right_col), grid))
}

fn get_gear_ratio(coord: Coord, grid: &Grid) -> Option<u32> {
    let (row, col) = coord;

    if grid.layout[row][col] != b'*' {
        return None;
    }

    let mut part_numbers = neighbours(coord, grid)
        .into_iter()
        .filter_map(|coord| get_part_number(coord, grid))
        .collect::<Vec<_>>();
    part_numbers.dedup();

    if part_numbers.len() == 2 {
        Some(part_numbers.into_iter().product())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 4361);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 467835);

        Ok(())
    }
}
