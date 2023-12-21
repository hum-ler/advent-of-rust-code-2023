use crate::day_12_part_1::*;

pub fn run(_input: &str) -> usize {
    todo!()
}

fn _arrangements(_pattern: &[Spring], _count: &[usize]) -> usize {
    todo!()
}

/// Partially match and consume any Spring::Any plus its count.
///
/// Return None if the pattern fails the matching. Otherwise, Some((pattern_left, count_left)) for
/// further matching.
fn _is_partial_match(_pattern: &[Spring], _count: &[usize]) -> Option<(Vec<Spring>, Vec<usize>)> {
    todo!()
}

fn _parse_line(input: &str) -> (Vec<Spring>, Vec<usize>) {
    let (pattern_part, counts_part) = input.split_once(' ').unwrap();

    let pattern = pattern_part
        .chars()
        .map(Spring::from)
        .collect::<Vec<Spring>>();
    let mut unfolded_pattern = vec![];
    unfolded_pattern.extend_from_slice(&pattern);
    unfolded_pattern.push(Spring::Any);
    unfolded_pattern.extend_from_slice(&pattern);
    unfolded_pattern.push(Spring::Any);
    unfolded_pattern.extend_from_slice(&pattern);
    unfolded_pattern.push(Spring::Any);
    unfolded_pattern.extend_from_slice(&pattern);
    unfolded_pattern.push(Spring::Any);
    unfolded_pattern.extend_from_slice(&pattern);

    let counts = counts_part
        .split(',')
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect::<Vec<usize>>();
    let mut unfolded_counts = vec![];
    unfolded_counts.extend_from_slice(&counts);
    unfolded_counts.extend_from_slice(&counts);
    unfolded_counts.extend_from_slice(&counts);
    unfolded_counts.extend_from_slice(&counts);
    unfolded_counts.extend_from_slice(&counts);

    (unfolded_pattern, unfolded_counts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
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

        assert_eq!(run(input), 525152);
    }
}
