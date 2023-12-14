use crate::clean_lines;

pub fn run(input: &str) -> usize {
    let grid = roll_north(parse_grid(input));

    let max_load = grid[0].len();
    transpose(&grid)
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|c| **c == 'O').count() * (max_load - i))
        .sum::<usize>()
}

/// Parses the input map and returns the grid of characters *in columns*.
pub(crate) fn parse_grid(input: &str) -> Vec<Vec<char>> {
    transpose(
        &clean_lines(input)
            .map(str::chars)
            .map(|line| line.collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    )
}

pub(crate) fn roll_north(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.iter()
        .map(|column| roll_column_north(column))
        .collect::<Vec<Vec<char>>>()
}

#[allow(clippy::mut_range_bound)]
fn roll_column_north(column: &[char]) -> Vec<char> {
    let mut rolled = Vec::from(column);

    let mut i = 0;
    'outer: while i < column.len() {
        // Find the next empty space.
        match rolled[i] {
            'O' | '#' => {
                i += 1;
                continue;
            }
            '.' => {
                // Find the next rolling stone.
                'inner: for j in i..column.len() {
                    match rolled[j] {
                        '.' => continue 'inner,
                        '#' => {
                            // If we hit an obstacle, skip ahead.
                            i = j + 1;
                            continue 'outer;
                        }
                        'O' => {
                            rolled.swap(i, j);
                            i += 1;
                            continue 'outer;
                        }
                        _ => panic!("Unexpected character '{}' in column", rolled[j]),
                    }
                }

                break;
            }
            _ => panic!("Unexpected character '{}' in column", rolled[i]),
        }
    }

    rolled
}

/// Converts from rows to columns, and vice versa.
pub(crate) fn transpose(rows: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut columns = rows[0]
        .iter()
        .map(|_| Vec::<char>::new())
        .collect::<Vec<Vec<char>>>();

    rows.iter().for_each(|row| {
        row.iter()
            .enumerate()
            .for_each(|(i, c)| columns[i].push(*c))
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

        assert_eq!(run(input), 136);
    }
}
