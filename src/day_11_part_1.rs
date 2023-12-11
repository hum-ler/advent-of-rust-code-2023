use std::collections::HashSet;

use itertools::Itertools;

use crate::clean_lines;

pub fn run(input: &str) -> usize {
    let lines = clean_lines(input).collect::<Vec<&str>>();
    let (width, height) = get_size(&lines);

    let galaxies = lines
        .into_iter()
        .enumerate()
        .map(|(row, line)| parse_line(line, row))
        .flatten()
        .collect::<Vec<Galaxy>>();

    let galaxies = expand_universe(&galaxies, width, height);

    galaxies
        .into_iter()
        .combinations(2)
        .map(|pair| distance(pair[0], pair[1]))
        .sum::<usize>()
}

pub(crate) fn get_size(lines: &[&str]) -> (usize, usize) {
    (lines.first().unwrap().len(), lines.len())
}

pub(crate) fn parse_line(input: &str, row: usize) -> Vec<Galaxy> {
    input
        .chars()
        .enumerate()
        .filter_map(|(column, token)| match token {
            '#' => Some((column, row)),
            _ => None,
        })
        .collect::<Vec<Galaxy>>()
}

fn expand_universe(galaxies: &[Galaxy], width: usize, height: usize) -> Vec<Galaxy> {
    let empty_columns = find_empty_columns(galaxies, width);
    let empty_rows = find_empty_rows(galaxies, height);

    galaxies
        .iter()
        .map(|galaxy| {
            (
                expand(galaxy.0, &empty_columns),
                expand(galaxy.1, &empty_rows),
            )
        })
        .collect::<Vec<Galaxy>>()
}

pub(crate) fn find_empty_columns(galaxies: &[Galaxy], width: usize) -> Vec<usize> {
    let unique_columns: HashSet<usize> = HashSet::from_iter(galaxies.iter().map(|galaxy| galaxy.0));
    let all_columns = HashSet::from_iter((0..width).into_iter());
    all_columns
        .difference(&unique_columns)
        .map(|column| *column)
        .collect::<Vec<usize>>()
}

pub(crate) fn find_empty_rows(galaxies: &[Galaxy], height: usize) -> Vec<usize> {
    let unique_rows: HashSet<usize> = HashSet::from_iter(galaxies.iter().map(|galaxy| galaxy.1));
    let all_rows = HashSet::from_iter((0..height).into_iter());
    all_rows
        .difference(&unique_rows)
        .map(|row| *row)
        .collect::<Vec<usize>>()
}

fn expand(value: usize, at: &[usize]) -> usize {
    value + at.iter().filter(|at| **at < value).count()
}

pub(crate) fn distance(galaxy: Galaxy, other: Galaxy) -> usize {
    galaxy.0.abs_diff(other.0) + galaxy.1.abs_diff(other.1)
}

pub(crate) type Galaxy = (usize, usize);

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

        assert_eq!(run(input), 374);
    }
}
