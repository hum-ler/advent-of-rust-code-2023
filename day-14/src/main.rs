use std::{collections::HashMap, str::FromStr};

use anyhow::Result;

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-14.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<usize> {
    let mut grid = Grid::from_str(input)?;

    grid.tilt_north();

    Ok(grid.load())
}

fn part_2(input: &str) -> Result<usize> {
    let mut grid = Grid::from_str(input)?;

    // Find the first cycle.
    let mut spins = 1;
    let mut history = HashMap::new();
    let cycle = loop {
        grid.spin();

        if history.contains_key(&grid.layout) {
            break spins - history[&grid.layout];
        } else {
            history.entry(grid.layout.clone()).or_insert(spins);
        }

        spins += 1;
    };

    // Complete the remaining spins.
    for _ in 0..(1000000000 - spins) % cycle {
        grid.spin();
    }

    Ok(grid.load())
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
    fn tilt_north(&mut self) {
        for col in 0..self.size {
            let mut empty_space = None;

            let mut row = 0;
            while row < self.size {
                match self.layout[row][col] {
                    b'.' => empty_space = empty_space.or(Some(row)),
                    b'#' => empty_space = None,
                    b'O' => {
                        if let Some(empty_row) = empty_space {
                            (self.layout[empty_row][col], self.layout[row][col]) =
                                (self.layout[row][col], self.layout[empty_row][col]);

                            empty_space = None;
                            row = empty_row + 1;
                            continue;
                        }
                    }
                    _ => (),
                }

                row += 1;
            }
        }
    }

    fn tilt_east(&mut self) {
        for row in 0..self.size {
            let mut empty_space = None;

            let mut col = self.size - 1;
            loop {
                match self.layout[row][col] {
                    b'.' => empty_space = empty_space.or(Some(col)),
                    b'#' => empty_space = None,
                    b'O' => {
                        if let Some(empty_col) = empty_space {
                            (self.layout[row][empty_col], self.layout[row][col]) =
                                (self.layout[row][col], self.layout[row][empty_col]);

                            empty_space = None;
                            col = empty_col - 1;
                            continue;
                        }
                    }
                    _ => (),
                }

                if col == 0 {
                    break;
                } else {
                    col -= 1;
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for col in 0..self.size {
            let mut empty_space = None;

            let mut row = self.size - 1;
            loop {
                match self.layout[row][col] {
                    b'.' => empty_space = empty_space.or(Some(row)),
                    b'#' => empty_space = None,
                    b'O' => {
                        if let Some(empty_row) = empty_space {
                            (self.layout[empty_row][col], self.layout[row][col]) =
                                (self.layout[row][col], self.layout[empty_row][col]);

                            empty_space = None;
                            row = empty_row - 1;
                            continue;
                        }
                    }
                    _ => (),
                }

                if row == 0 {
                    break;
                } else {
                    row -= 1;
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for row in 0..self.size {
            let mut empty_space = None;

            let mut col = 0;
            while col < self.size {
                match self.layout[row][col] {
                    b'.' => empty_space = empty_space.or(Some(col)),
                    b'#' => empty_space = None,
                    b'O' => {
                        if let Some(empty_col) = empty_space {
                            (self.layout[row][empty_col], self.layout[row][col]) =
                                (self.layout[row][col], self.layout[row][empty_col]);

                            empty_space = None;
                            col = empty_col + 1;
                            continue;
                        }
                    }
                    _ => (),
                }

                col += 1;
            }
        }
    }

    fn load(&self) -> usize {
        self.layout
            .iter()
            .enumerate()
            .flat_map(|(row, bytes)| {
                bytes.iter().filter_map(move |byte| {
                    if *byte == b'O' {
                        Some(self.size - row)
                    } else {
                        None
                    }
                })
            })
            .sum()
    }

    fn spin(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 136);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 64);

        Ok(())
    }
}
