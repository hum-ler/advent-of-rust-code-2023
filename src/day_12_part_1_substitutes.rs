// We can brute-force this by subtituting the ?s with the correct number of remaining #s:
// - count(substitute-#s) = length(row) - total(#s)
// - count(substitite-.s) = count(?s) - count(substitute-#s)
//
// If we enumerate all permutations of ? replacements, substitute into the original pattern, we
// can then figure out the derived #s.
//
// Match these against the original #s to count the number of possible arrangements.
//
// Unfortunately, permutate/unique takes ridiculously long. So we just brute-force all possible ?
// replacements.

use crate::clean_lines;

pub fn run(input: &str) -> usize {
    clean_lines(input)
        .map(parse_line)
        .map(|(pattern, count)| arrangements(&pattern, &count))
        .sum()
}

fn arrangements(pattern: &[Spring], count: &[usize]) -> usize {
    // Terminate once we can run is_match().
    if !pattern.contains(&Spring::Any) {
        return if is_match(pattern, count) { 1 } else { 0 };
    }

    let i = pattern.iter().position(|spring| *spring == Spring::Any).unwrap();

    let mut operational_pattern = Vec::with_capacity(pattern.len());
    operational_pattern.extend_from_slice(pattern);
    operational_pattern[i] = Spring::Operational;

    let mut damaged_pattern = Vec::with_capacity(pattern.len());
    damaged_pattern.extend_from_slice(pattern);
    damaged_pattern[i] = Spring::Damaged;

    arrangements(&operational_pattern, count) + arrangements(&damaged_pattern, count)
}

fn is_match(pattern: &[Spring], count: &[usize]) -> bool {
    pattern_count(pattern) == count
}

fn pattern_count(pattern: &[Spring]) -> Vec<usize> {
    let mut count = vec![];

    let mut consecutive_damaged = 0;
    (0..pattern.len()).for_each(|i| match pattern[i] {
        Spring::Operational => {
            if consecutive_damaged > 0 {
                count.push(consecutive_damaged);
                consecutive_damaged = 0;
            }
        }
        Spring::Damaged => {
            consecutive_damaged += 1;
        }
        Spring::Any => unreachable!(),
    });

    if consecutive_damaged > 0 {
        count.push(consecutive_damaged);
    }

    count
}

fn parse_line(input: &str) -> (Vec<Spring>, Vec<usize>) {
    let (pattern_part, counts_part) = input.split_once(' ').unwrap();

    let pattern = pattern_part.chars().map(Spring::from).collect();

    let counts = counts_part
        .split(',')
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect::<Vec<usize>>();

    (pattern, counts)
}

#[derive(Clone, Copy, PartialEq)]
enum Spring {
    Any,
    Operational,
    Damaged,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '?' => Self::Any,
            '.' => Self::Operational,
            '#' => Self::Damaged,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        ";

        assert_eq!(run(input), 21);
    }

    #[test]
    fn check_counting() {
        // #.#.### 1,1,3
        assert_eq!(
            pattern_count(&[
                Spring::Damaged,
                Spring::Operational,
                Spring::Damaged,
                Spring::Operational,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
            ]),
            vec![1, 1, 3]
        );

        // .#...#....###. 1,1,3
        assert_eq!(
            pattern_count(&[
                Spring::Operational,
                Spring::Damaged,
                Spring::Operational,
                Spring::Operational,
                Spring::Operational,
                Spring::Damaged,
                Spring::Operational,
                Spring::Operational,
                Spring::Operational,
                Spring::Operational,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Operational,
            ]),
            vec![1, 1, 3]
        );

        // .#.###.#.###### 1,3,1,6
        assert_eq!(
            pattern_count(&[
                Spring::Operational,
                Spring::Damaged,
                Spring::Operational,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Operational,
                Spring::Damaged,
                Spring::Operational,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
            ]),
            vec![1, 3, 1, 6]
        );

        // ####.#...#... 4,1,1
        assert_eq!(
            pattern_count(&[
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Operational,
                Spring::Damaged,
                Spring::Operational,
                Spring::Operational,
                Spring::Operational,
                Spring::Damaged,
                Spring::Operational,
                Spring::Operational,
                Spring::Operational,
            ]),
            vec![4, 1, 1]
        );

        // #....######..#####. 1,6,5
        assert_eq!(
            pattern_count(&[
                Spring::Damaged,
                Spring::Operational,
                Spring::Operational,
                Spring::Operational,
                Spring::Operational,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Operational,
                Spring::Operational,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Operational,
            ]),
            vec![1, 6, 5]
        );

        // .###.##....# 3,2,1
        assert_eq!(
            pattern_count(&[
                Spring::Operational,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Operational,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Operational,
                Spring::Operational,
                Spring::Operational,
                Spring::Operational,
                Spring::Damaged,
            ]),
            vec![3, 2, 1]
        );
    }
}
