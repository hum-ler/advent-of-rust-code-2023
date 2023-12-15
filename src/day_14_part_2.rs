use crate::day_14_part_1::*;

pub fn run(input: &str) -> usize {
    let mut grid = parse_grid(input);

    // After a couple hundred spins, the grid eventually settles down to a cycle of 7 states. Over
    // here we just do a thousand spins. Final spin: 1000 mod 7 = 1000000000 mod 7.
    for _ in 0..1000 {
        grid = spin_cycle(grid);
    }

    get_load(&grid)
}

fn get_load(grid: &[Vec<char>]) -> usize {
    let max_load = grid[0].len();
    transpose(grid)
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|c| **c == 'O').count() * (max_load - i))
        .sum::<usize>()
}

fn spin_cycle(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    rotate_clockwise(roll_north(rotate_clockwise(roll_north(rotate_clockwise(
        roll_north(rotate_clockwise(roll_north(grid))),
    )))))
}

fn rotate_clockwise(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut columns = grid[0]
        .iter()
        .map(|_| Vec::<char>::new())
        .collect::<Vec<Vec<char>>>();

    let prev_grid_height = grid.len();
    let prev_grid_width = grid[0].len();

    (0..prev_grid_width).for_each(|i| {
        (0..prev_grid_height).for_each(|j| {
            columns[i].push(grid[j][prev_grid_width - 1 - i]);
        });
    });

    columns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
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

        assert_eq!(run(input), 64);
    }
}
