use crate::clean_lines;

pub fn run(input: &str) -> usize {
    clean_lines(input)
        .map(parse_line)
        .map(|(pattern, counts)| arrangements(&pattern, &counts))
        .sum()
}

fn optimize_pattern<'a, 'b>(
    pattern: &'a [char],
    counts: &'b [usize],
) -> Option<(&'a [char], &'b [usize])> {
    let mut pattern_left_cutoff = 0;
    let mut counts_left_cutoff = 0;
    let mut pattern_right_cutoff = pattern.len() - 1;
    let mut counts_right_cutoff = counts.len() - 1;

    // Consume as much as possible from lhs.
    let mut pattern_pointer = 0;
    let mut consecutive_damaged = 0;
    loop {
        if pattern_pointer > pattern_right_cutoff || counts_left_cutoff > counts_right_cutoff {
            break;
        }

        match pattern[pattern_pointer] {
            '#' => {
                consecutive_damaged += 1;

                if let Some(&count) = counts.get(counts_left_cutoff) {
                    if consecutive_damaged > count {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            '.' => {
                if consecutive_damaged > 0 {
                    if consecutive_damaged != counts[counts_left_cutoff] {
                        return None;
                    }

                    counts_left_cutoff += 1;
                    consecutive_damaged = 0;
                }

                pattern_left_cutoff = pattern_pointer + 1;
            }
            '?' => {
                if consecutive_damaged > 0 {
                    if let Some(&count) = counts.get(counts_left_cutoff) {
                        if consecutive_damaged > count {
                            return None;
                        }

                        if consecutive_damaged == count {
                            // Can only be .
                            counts_left_cutoff += 1;
                            consecutive_damaged = 0;

                            pattern_left_cutoff = pattern_pointer + 1;
                        }

                        if consecutive_damaged < count {
                            // Can only be #
                            consecutive_damaged += 1;

                            if consecutive_damaged > count {
                                return None;
                            }
                        }
                    } else {
                        return None;
                    }
                }

                // Cannot tell.
                break;
            }
            _ => unreachable!(),
        }

        pattern_pointer += 1;
    }

    // Consume as much as possible from rhs.
    let mut pattern_pointer = pattern_right_cutoff;
    let mut consecutive_damaged = 0;
    loop {
        if pattern_pointer < pattern_left_cutoff || counts_right_cutoff < counts_left_cutoff {
            break;
        }

        match pattern[pattern_pointer] {
            '#' => {
                consecutive_damaged += 1;

                if let Some(&count) = counts.get(counts_right_cutoff) {
                    if consecutive_damaged > count {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            '.' => {
                if consecutive_damaged > 0 {
                    if consecutive_damaged != counts[counts_right_cutoff] {
                        return None;
                    }

                    counts_right_cutoff -= 1;
                    consecutive_damaged = 0;
                }

                pattern_right_cutoff = pattern_pointer - 1;
            }
            '?' => {
                if consecutive_damaged > 0 {
                    if let Some(&count) = counts.get(counts_right_cutoff) {
                        if consecutive_damaged > count {
                            return None;
                        }

                        if consecutive_damaged == count {
                            // Can only be .
                            counts_right_cutoff -= 1;
                            consecutive_damaged = 0;

                            pattern_right_cutoff = pattern_pointer - 1;
                        }

                        if consecutive_damaged < count {
                            // Can only be #
                            consecutive_damaged += 1;

                            if consecutive_damaged > count {
                                return None;
                            }
                        }
                    } else {
                        return None;
                    }
                }

                // Cannot tell.
                break;
            }
            _ => unreachable!(),
        }

        if pattern_pointer == 0 {
            break;
        } else {
            pattern_pointer -= 1;
        }
    }

    // Quick check for pattern length.
    if counts_left_cutoff < counts_right_cutoff
        && pattern_right_cutoff + 1 - pattern_left_cutoff
            < counts[counts_left_cutoff..=counts_right_cutoff]
                .iter()
                .copied()
                .reduce(|acc, c| acc + 1 + c)
                .unwrap()
    {
        return None;
    }

    Some((
        &pattern[pattern_left_cutoff..=pattern_right_cutoff],
        &counts[counts_left_cutoff..=counts_right_cutoff],
    ))
}

pub(crate) fn arrangements(pattern: &[char], counts: &[usize]) -> usize {
    let optimized_pattern = optimize_pattern(pattern, counts);

    // Terminate if optimize_pattern() discovers a problem.
    if optimized_pattern.is_none() {
        return 0;
    }

    let (pattern, count) = optimized_pattern.unwrap();

    // Terminate if count is already empty.
    if count.is_empty() {
        return if !pattern.contains(&'#') { 1 } else { 0 };
    }

    // Terminate if there can be no more substitution.
    // This also takes care of the case where pattern is already empty.
    if !pattern.contains(&'?') {
        return if is_match(pattern, count) { 1 } else { 0 };
    }

    // Terminate early if we are down to all # and ?.
    if count.len() == 1 && pattern.len() == count[0] {
        return if !pattern.contains(&'.') { 1 } else { 0 };
    }

    let i = pattern.iter().position(|spring| *spring == '?').unwrap();

    let mut operational_pattern = Vec::with_capacity(pattern.len());
    operational_pattern.extend_from_slice(pattern);
    operational_pattern[i] = '.';

    let mut damaged_pattern = Vec::with_capacity(pattern.len());
    damaged_pattern.extend_from_slice(pattern);
    damaged_pattern[i] = '#';

    arrangements(&operational_pattern, count) + arrangements(&damaged_pattern, count)
}

fn is_match(pattern: &[char], counts: &[usize]) -> bool {
    pattern_counts(pattern) == counts
}

fn pattern_counts(pattern: &[char]) -> Vec<usize> {
    let mut counts = vec![];

    let mut consecutive_damaged = 0;
    (0..pattern.len()).for_each(|i| match pattern[i] {
        '.' => {
            if consecutive_damaged > 0 {
                counts.push(consecutive_damaged);
                consecutive_damaged = 0;
            }
        }
        '#' => {
            consecutive_damaged += 1;
        }
        _ => unreachable!(),
    });

    if consecutive_damaged > 0 {
        counts.push(consecutive_damaged);
    }

    counts
}

fn parse_line(input: &str) -> (Vec<char>, Vec<usize>) {
    let (pattern_part, counts_part) = input.split_once(' ').unwrap();

    let pattern = pattern_part.chars().collect();

    let counts = counts_part
        .split(',')
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect::<Vec<usize>>();

    (pattern, counts)
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
}
