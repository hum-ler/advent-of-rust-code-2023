use std::collections::VecDeque;

pub fn run(input: &str) -> usize {
    run_steps(input, 26501365)
}

pub fn run_steps(input: &str, steps: usize) -> usize {
    // Following the ideas from https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21

    let (grid, start_pos, grid_size) = convert_input_to_grid(input);

    let (full_odd, full_even) = {
        let mut center_grid = grid.clone();
        flood_fill((start_pos.0, start_pos.1), &mut center_grid, grid_size);
        (
            count_odd(&center_grid, usize::MAX),
            count_even(&center_grid, usize::MAX),
        )
    };

    let n_odd = {
        let mut n_grid = grid.clone();
        flood_fill((grid_size.0 - 1, start_pos.1), &mut n_grid, grid_size);
        count_even(&n_grid, grid_size.0 - 1) // FIXME: why even?
    };

    let (ne_odd, ne_even) = {
        let mut ne_grid = grid.clone();
        flood_fill((grid_size.0 - 1, 0), &mut ne_grid, grid_size);
        (
            count_odd(&ne_grid, grid_size.0 - 1 + start_pos.1),
            count_even(&ne_grid, start_pos.0),
        )
    };

    let e_odd = {
        let mut e_grid = grid.clone();
        flood_fill((start_pos.0, 0), &mut e_grid, grid_size);
        count_even(&e_grid, grid_size.1 - 1) // FIXME: why even?
    };

    let (se_odd, se_even) = {
        let mut se_grid = grid.clone();
        flood_fill((0, 0), &mut se_grid, grid_size);
        (
            count_odd(&se_grid, grid_size.0 - 1 + start_pos.1),
            count_even(&se_grid, start_pos.0),
        )
    };

    let s_odd = {
        let mut s_grid = grid.clone();
        flood_fill((0, start_pos.1), &mut s_grid, grid_size);
        count_even(&s_grid, grid_size.0 - 1) // FIXME: why even?
    };

    let (sw_odd, sw_even) = {
        let mut sw_grid = grid.clone();
        flood_fill((0, grid_size.1 - 1), &mut sw_grid, grid_size);
        (
            count_odd(&sw_grid, grid_size.0 - 1 + start_pos.1),
            count_even(&sw_grid, start_pos.0),
        )
    };

    let w_odd = {
        let mut w_grid = grid.clone();
        flood_fill((start_pos.0, grid_size.1 - 1), &mut w_grid, grid_size);
        count_even(&w_grid, grid_size.1 - 1) // FIXME: why even?
    };

    let (nw_odd, nw_even) = {
        let mut nw_grid = grid.clone();
        flood_fill((grid_size.0 - 1, grid_size.1 - 1), &mut nw_grid, grid_size);
        (
            count_odd(&nw_grid, grid_size.0 - 1 + start_pos.1),
            count_even(&nw_grid, start_pos.0),
        )
    };

    let n: usize = (steps - start_pos.1) / grid_size.1;

    (n - 1).pow(2) * full_odd
        + n.pow(2) * full_even
        + (n - 1) * (ne_odd + se_odd + sw_odd + nw_odd)
        + n * (ne_even + se_even + sw_even + nw_even)
        + n_odd
        + e_odd
        + s_odd
        + w_odd
}

#[derive(Clone, PartialEq)]
enum Plot {
    Rock,
    Odd(usize),
    Even(usize),
    Unvisited,
}

type Coord = (usize, usize);

type GridSize = (usize, usize);

fn convert_input_to_grid(input: &str) -> (Vec<Vec<Plot>>, Coord, GridSize) {
    let mut start_pos = (0, 0);

    let grid = input
        .trim()
        .split_terminator("\n")
        .enumerate()
        .map(|(row, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(col, byte)| match *byte {
                    b'#' => Plot::Rock,
                    b'S' => {
                        start_pos = (row, col);
                        Plot::Unvisited
                    }
                    _ => Plot::Unvisited,
                })
                .collect()
        })
        .collect::<Vec<_>>();

    let grid_size = (grid.len(), grid.first().map_or(0, Vec::len));

    (grid, start_pos, grid_size)
}

fn flood_fill(seed: Coord, grid: &mut [Vec<Plot>], grid_size: GridSize) {
    let (row_count, col_count) = grid_size;

    let mut check_queue: VecDeque<(Coord, usize)> = VecDeque::default();
    check_queue.push_back((seed, 0));

    while let Some(pos) = check_queue.pop_front() {
        let ((row, col), steps) = pos;
        assert!(row < row_count);
        assert!(col < col_count);

        match grid[row][col] {
            Plot::Rock | Plot::Odd(_) | Plot::Even(_) => (),
            Plot::Unvisited => {
                // Update this Plot.
                grid[row][col] = if steps % 2 == 0 {
                    Plot::Even(steps)
                } else {
                    Plot::Odd(steps)
                };

                // n
                if row > 0 {
                    check_queue.push_back(((row - 1, col), steps + 1));
                }

                // e
                if col < col_count - 1 {
                    check_queue.push_back(((row, col + 1), steps + 1));
                }

                // s
                if row < row_count - 1 {
                    check_queue.push_back(((row + 1, col), steps + 1));
                }

                // w
                if col > 0 {
                    check_queue.push_back(((row, col - 1), steps + 1));
                }
            }
        }
    }
}

fn count_odd(grid: &[Vec<Plot>], upper_limit: usize) -> usize {
    grid.iter()
        .map(|row| {
            row.iter()
                .filter(|plot| matches!(plot, Plot::Odd(x) if *x <= upper_limit))
                .count()
        })
        .sum()
}

fn count_even(grid: &[Vec<Plot>], upper_limit: usize) -> usize {
    grid.iter()
        .map(|row| {
            row.iter()
                .filter(|plot| matches!(plot, Plot::Even(x) if *x <= upper_limit))
                .count()
        })
        .sum()
}

fn _print_grid(grid: &[Vec<Plot>]) {
    for row in grid {
        for plot in row {
            match plot {
                Plot::Odd(x) => print!("{:^3} ", x),
                Plot::Even(x) => print!("{:^3} ", x),
                Plot::Rock => print!(" #  "),
                Plot::Unvisited => (),
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    #[ignore = "run_steps() will only work if steps ends exactly at window boundary."]
    #[test]
    fn run_example() {
        use super::*;

        let input = r"
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

        assert_eq!(run_steps(input, 5000), 16733044);
    }
}
