use itertools::Itertools;

use crate::{clean_lines, day_11_part_1::*};

pub fn run(input: &str) -> usize {
    run_with_expansion_size(input, 1000000)
}

fn run_with_expansion_size(input: &str, size: usize) -> usize {
    let lines = clean_lines(input).collect::<Vec<&str>>();
    let (width, height) = get_size(&lines);

    let galaxies = lines
        .into_iter()
        .enumerate()
        .map(|(row, line)| parse_line(line, row))
        .flatten()
        .collect::<Vec<Galaxy>>();

    let galaxies = expand_universe(&galaxies, size, width, height);

    galaxies
        .into_iter()
        .combinations(2)
        .map(|pair| distance(pair[0], pair[1]))
        .sum::<usize>()
}

fn expand_universe(galaxies: &[Galaxy], size: usize, width: usize, height: usize) -> Vec<Galaxy> {
    let empty_columns = find_empty_columns(galaxies, width);
    let empty_rows = find_empty_rows(galaxies, height);

    galaxies
        .iter()
        .map(|galaxy| {
            (
                expand(galaxy.0, size, &empty_columns),
                expand(galaxy.1, size, &empty_rows),
            )
        })
        .collect::<Vec<Galaxy>>()
}

fn expand(value: usize, size: usize, at: &[usize]) -> usize {
    // Note that size includes the original empty row or column.
    value + at.iter().filter(|at| **at < value).count() * (size - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
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

        assert_eq!(run_with_expansion_size(input, 10), 1030);
        assert_eq!(run_with_expansion_size(input, 100), 8410);
    }
}
