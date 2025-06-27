use std::{collections::HashSet, str::FromStr};

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-10.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<usize> {
    let grid = Grid::from_str(input)?;

    Ok(grid.as_loop_coords()?.len() / 2)
}

fn part_2(input: &str) -> Result<usize> {
    let grid = Grid::from_str(input)?;
    let loop_coords = grid.as_loop_coords()?;

    let mut enclosed_tiles = 0;
    for row in 0..grid.size.0 {
        let mut is_within_loop = false;
        let mut top_exit = false;
        let mut bottom_exit = false;
        for col in 0..grid.size.1 {
            if loop_coords.contains(&(row, col)) {
                // Check if we cross the loop circuit. Be careful of loop U-turns.
                match grid.layout[row][col] {
                    b'|' => {
                        top_exit = true;
                        bottom_exit = true;
                    }
                    b'L' | b'J' => {
                        top_exit = !top_exit;
                    }
                    b'F' | b'7' => {
                        bottom_exit = !bottom_exit;
                    }
                    _ => (),
                }

                if top_exit && bottom_exit {
                    top_exit = false;
                    bottom_exit = false;
                    is_within_loop = !is_within_loop;
                }
            } else if is_within_loop {
                enclosed_tiles += 1;
            }
        }
    }

    Ok(enclosed_tiles)
}

/// (row, col)
type Coord = (usize, usize);

/// (rows, cols)
type GridSize = (usize, usize);

struct Grid {
    layout: Vec<Vec<u8>>,
    start: Coord,
    size: GridSize,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        let rows = lines.len();
        let cols = lines.first().map_or(0, |line| line.len());
        let size = (rows, cols);

        let row = lines
            .iter()
            .position(|row| row.contains('S'))
            .ok_or(anyhow!("Cannot find start row"))?;
        let col = lines[row]
            .find('S')
            .ok_or(anyhow!("Cannot find start col"))?;
        let start = (row, col);

        let layout = lines
            .into_iter()
            .map(|line| line.bytes().collect())
            .collect();

        Ok(Self {
            layout,
            start,
            size,
        })
    }
}

impl Grid {
    fn as_loop_coords(&self) -> Result<HashSet<Coord>> {
        let start_exits = self.find_pipe_exits(self.start)?;

        let mut loop_coords = HashSet::new();
        loop_coords.insert(self.start);
        loop_coords.extend(start_exits);

        let mut prev_coord = self.start;
        let mut coord = start_exits[0];
        while coord != start_exits[1] {
            (prev_coord, coord) = (coord, self.traverse_pipe(coord, prev_coord)?);
            loop_coords.insert(coord);
        }

        Ok(loop_coords)
    }

    fn find_pipe_exits(&self, coord: Coord) -> Result<[Coord; 2]> {
        let (row, col) = coord;

        match self.layout[row][col] {
            b'|' if row > 0 && row < self.size.0 - 1 => Ok([(row - 1, col), (row + 1, col)]),
            b'-' if col > 0 && col < self.size.1 - 1 => Ok([(row, col + 1), (row, col - 1)]),
            b'L' if row > 0 && col < self.size.1 - 1 => Ok([(row - 1, col), (row, col + 1)]),
            b'J' if row > 0 && col > 0 => Ok([(row - 1, col), (row, col - 1)]),
            b'7' if row < self.size.0 - 1 && col > 0 => Ok([(row + 1, col), (row, col - 1)]),
            b'F' if row < self.size.0 - 1 && col < self.size.1 - 1 => {
                Ok([(row, col + 1), (row + 1, col)])
            }
            b'S' => {
                let mut exits = Vec::new();

                if row > 0 && matches!(self.layout[row - 1][col], b'|' | b'7' | b'F') {
                    exits.push((row - 1, col));
                }
                if col < self.size.1 - 1 && matches!(self.layout[row][col + 1], b'-' | b'J' | b'7')
                {
                    exits.push((row, col + 1));
                }
                if row < self.size.0 - 1 && matches!(self.layout[row + 1][col], b'|' | b'L' | b'J')
                {
                    exits.push((row + 1, col));
                }
                if col > 0 && matches!(self.layout[row][col - 1], b'-' | b'L' | b'F') {
                    exits.push((row, col - 1));
                }

                if exits.len() == 2 {
                    Ok([exits[0], exits[1]])
                } else {
                    Err(anyhow!("Cannot determine exits for S: {:?}", coord))
                }
            }
            _ => Err(anyhow!(
                "Invalid pipe {}: {:?}",
                self.layout[row][col],
                coord
            )),
        }
    }

    fn traverse_pipe(&self, coord: Coord, prev_coord: Coord) -> Result<Coord> {
        let pipe_exits = self.find_pipe_exits(coord)?;

        match pipe_exits {
            [entrance, exit] | [exit, entrance] if prev_coord == entrance => Ok(exit),
            _ => Err(anyhow!("Invalid prev coord")),
        }
    }
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    #[test]
    fn example_1a() -> Result<()> {
        let example = r"
.....
.S-7.
.|.|.
.L-J.
.....
";

        assert_eq!(part_1(trim_newlines(example))?, 4);

        Ok(())
    }

    #[test]
    fn example_1b() -> Result<()> {
        let example = r"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

        assert_eq!(part_1(trim_newlines(example))?, 8);

        Ok(())
    }

    #[test]
    fn example_2a() -> Result<()> {
        let example = r"
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

        assert_eq!(part_2(trim_newlines(example))?, 4);

        Ok(())
    }

    #[test]
    fn example_2b() -> Result<()> {
        let example = r"
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

        assert_eq!(part_2(trim_newlines(example))?, 4);

        Ok(())
    }

    #[test]
    fn example_2c() -> Result<()> {
        let example = r"
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

        assert_eq!(part_2(trim_newlines(example))?, 8);

        Ok(())
    }

    #[test]
    fn example_2d() -> Result<()> {
        let example = r"
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

        assert_eq!(part_2(trim_newlines(example))?, 10);

        Ok(())
    }
}
