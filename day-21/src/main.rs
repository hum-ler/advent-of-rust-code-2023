use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-21.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<usize> {
    reachable_plots_in_steps(input, 64)
}

fn part_2(input: &str) -> Result<usize> {
    reachable_plots_in_26501365_steps(input)
}

/// (row, col)
type Coord = (usize, usize);

struct Grid {
    rocks: HashSet<Coord>,
    start: Coord,
    size: usize,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        let size = lines.len();

        let Some(row) = lines.iter().position(|line| line.contains("S")) else {
            return Err(anyhow!("Cannot find row with S"));
        };
        let Some(col) = lines[row].find("S") else {
            return Err(anyhow!("Cannot find col that is S"));
        };
        let start = (row, col);

        let rocks = lines
            .into_iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.bytes().enumerate().filter_map(move |(col, byte)| {
                    if byte == b'#' { Some((row, col)) } else { None }
                })
            })
            .collect::<HashSet<_>>();

        Ok(Self { rocks, start, size })
    }
}

fn reachable_plots_in_steps(input: &str, steps: u32) -> Result<usize> {
    // Each garden plot is either reachable on an odd or even step, never both. We can map out the
    // shortest path to each plot, and if it is an "odd" plot, it can only be reached on an odd
    // step, and the same goes for "even" plots.

    let grid = Grid::from_str(input)?;

    Ok(compute_shortest_distance(grid.start, 0, &grid, Some(steps))
        .into_iter()
        .map(|row| {
            row.into_iter()
                .flatten()
                .filter(|distance| distance & 1 == steps & 1)
                .count()
        })
        .sum())
}

fn compute_shortest_distance(
    start: Coord,
    distance: u32,
    grid: &Grid,
    cutoff: Option<u32>,
) -> Vec<Vec<Option<u32>>> {
    let mut shortest_distances = vec![vec![None; grid.size]; grid.size];

    flood_fill_shortest_distance(&mut shortest_distances, start, distance, cutoff, grid);

    shortest_distances
}

fn flood_fill_shortest_distance(
    shortest_distances: &mut [Vec<Option<u32>>],
    coord: Coord,
    distance: u32,
    cutoff: Option<u32>,
    grid: &Grid,
) {
    // Use bfs instead of dfs to avoid having to check for and overwrite longer distances.

    let mut flood_queue = VecDeque::from([(coord, distance)]);
    while let Some((coord, distance)) = flood_queue.pop_front() {
        if let Some(cutoff) = cutoff {
            if distance > cutoff {
                continue;
            }
        }

        let (row, col) = coord;
        if shortest_distances[row][col].is_some() {
            continue;
        }

        shortest_distances[row][col] = Some(distance);

        let distance = distance + 1;
        if row > 0 && !grid.rocks.contains(&(row - 1, col)) {
            flood_queue.push_back(((row - 1, col), distance));
        }
        if col < grid.size - 1 && !grid.rocks.contains(&(row, col + 1)) {
            flood_queue.push_back(((row, col + 1), distance));
        }
        if row < grid.size - 1 && !grid.rocks.contains(&(row + 1, col)) {
            flood_queue.push_back(((row + 1, col), distance));
        }
        if col > 0 && !grid.rocks.contains(&(row, col - 1)) {
            flood_queue.push_back(((row, col - 1), distance));
        }
    }
}

fn reachable_plots_in_26501365_steps(input: &str) -> std::result::Result<usize, anyhow::Error> {
    // The input is a sq of 131 x 131, with S right at the centre: (65, 65). There are straight
    // empty lines (garden plots) that extend from S to the edge of the grid in all 4 directions. In
    // addition, the border of the input is also all empty.
    //
    // The target num of steps (26501365) happens to be 65 + 202300 * 131. So if we start from S and
    // keep walking towards the right:
    // ... S - 65 -->|<-- 202299 * 131 -->|<-- 131 -->|
    // we would reach 202300 grids beyond the starting grid, of which 202299 can be considered fully
    // covered, while the right-most grid we can cover up to kind of an arrow-shape. The same goes
    // for all 4 directions.
    //
    // Consider what happens when we cross the boundary from one grid to another: since the starting
    // point is an odd num (65) from the edge, and the size of the grid is also an odd num (131),
    // each time we move from one grid to the next, the num of reachable pos "switches" from odd pos
    // to even pos, or vice versa. So, if S is the set of odd pos (the target num is an odd num),
    // and C is the complement of S, and sub_S_W, sub_S_E are proper subsets of S:
    // |<-- 131 -->|<-- 202299 * 131 -->|<- 65 - S - 65 -->|<-- 202299 * 131 -->|<-- 131 -->|
    // |  sub_S_W  |  C, S, C, ... , C  |        S         |  C, S, C, ... , C  |  sub_S_E  |
    // The same applies to the vertical axis from N to S.
    //
    // Finally, let's consider the 45-deg diagonals, using a simplified case:
    //          sub_C_NW sub_S_N  sub_C_NE
    // sub_C_NW sub_S_NW     C    sub_S_NE sub_C_NE
    // sub_S_W      C        S        C    sub_S_E
    // sub_C_SW sub_S_SW     C    sub_S_SE sub_C_SE
    //          sub_C_SW sub_S_S  sub_C_SE
    // Generalising this to n grids between starting S and sub_S_?:
    // - count(S) = n^2
    // - count(C) = (n + 1)^2
    // - count(sub_S_?) = 1
    // - count(sub_S_??) = n
    // - count(sub_C_??) = n + 1
    // where, tracing when a grid starts "spreading":
    // - sub_S_? (arrow) start from the centre of the edge as an odd num and stops after 131 steps.
    // - sub_C_?? (triangle) starts from the corner as an even num and stops after 65 steps.
    // - sub_S_?? starts from the same corner as an even num and stops after 65 + 131 steps.
    //
    // There is probably some way to combine sub_S_?? with from sub_C_?? from the opposite end.

    let grid = Grid::from_str(input)?;

    let shortest_distances = compute_shortest_distance(grid.start, 0, &grid, None);
    let s = count_odd_elements(&shortest_distances);
    let c = count_even_elements(&shortest_distances);

    let s_cardinal = [
        (grid.size - 1, grid.size / 2),
        (grid.size / 2, 0),
        (0, grid.size / 2),
        (grid.size / 2, grid.size - 1),
    ]
    .into_iter()
    .map(|start| compute_shortest_distance(start, 1, &grid, Some(131)))
    .map(|shortest_distances| count_odd_elements(&shortest_distances))
    .sum::<usize>();

    let s_diagonal = [
        (grid.size - 1, 0),
        (0, 0),
        (0, grid.size - 1),
        (grid.size - 1, grid.size - 1),
    ]
    .into_iter()
    .map(|start| compute_shortest_distance(start, 0, &grid, Some(196)))
    .map(|shortest_distances| count_odd_elements(&shortest_distances))
    .sum::<usize>();

    let c_diagonal = [
        (grid.size - 1, 0),
        (0, 0),
        (0, grid.size - 1),
        (grid.size - 1, grid.size - 1),
    ]
    .into_iter()
    .map(|start| compute_shortest_distance(start, 0, &grid, Some(65)))
    .map(|shortest_distances| count_even_elements(&shortest_distances))
    .sum::<usize>();

    let n = 202299;
    Ok(s * n * n + c * (n + 1) * (n + 1) + s_cardinal + s_diagonal * n + c_diagonal * (n + 1))
}

fn count_odd_elements(shortest_distances: &[Vec<Option<u32>>]) -> usize {
    shortest_distances
        .iter()
        .map(|row| {
            row.iter()
                .flatten()
                .filter(|&distance| *distance & 1 == 1)
                .count()
        })
        .sum()
}

fn count_even_elements(shortest_distances: &[Vec<Option<u32>>]) -> usize {
    shortest_distances
        .iter()
        .map(|row| {
            row.iter()
                .flatten()
                .filter(|&distance| *distance & 1 == 0)
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(reachable_plots_in_steps(trim_newlines(EXAMPLE), 6)?, 16);

        Ok(())
    }
}
