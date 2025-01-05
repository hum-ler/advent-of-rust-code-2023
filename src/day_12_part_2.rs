use std::collections::HashMap;

use crate::clean_lines;

pub fn run(input: &str) -> usize {
    let conditions = clean_lines(input)
        .map(split_line_into_condition)
        .collect::<Vec<_>>();

    let mut cache: HashMap<(String, Vec<usize>, usize), usize> = HashMap::default();

    conditions
        .into_iter()
        .map(|condition| count_arrangements(condition.0, condition.1, 0, &mut cache))
        .sum()
}

type Condition = (String, Vec<usize>);

fn split_line_into_condition(input: &str) -> Condition {
    let (pattern, counts_part) = input.split_once(' ').unwrap();

    let pattern = format!(
        "{}?{}?{}?{}?{}.", // The extra '.' helps to trigger the collection of residuals
        pattern, pattern, pattern, pattern, pattern
    );

    let counts_part = format!(
        "{},{},{},{},{}",
        counts_part, counts_part, counts_part, counts_part, counts_part
    );

    let counts = counts_part
        .split_terminator(',')
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    (pattern.to_string(), counts)
}

/// Counts the number of possible arrangments.
fn count_arrangements(
    pattern: String,
    counts: Vec<usize>,
    residual: usize,
    cache: &mut HashMap<(String, Vec<usize>, usize), usize>,
) -> usize {
    if cache.contains_key(&(pattern.clone(), counts.clone(), residual)) {
        return cache[&(pattern, counts, residual)];
    }

    // Base case.
    if pattern.is_empty() {
        // If there are no more counts, then we have found a solution.
        return counts.is_empty() as usize;
    }

    let total_count = match pattern.as_bytes()[0] {
        // Extend the residuals and carry on.
        b'#' => count_arrangements(
            String::from(&pattern[1..]),
            counts.clone(),
            residual + 1,
            cache,
        ),
        b'.' => {
            if residual > 0 {
                // If there is residual and it doesn't match up to count, we can reject this thread.
                if !counts.is_empty() && counts[0] == residual {
                    count_arrangements(
                        String::from(&pattern[1..]),
                        Vec::from(&counts[1..]),
                        0,
                        cache,
                    )
                } else {
                    0
                }
            } else {
                count_arrangements(String::from(&pattern[1..]), counts.clone(), 0, cache)
            }
        }
        b'?' => {
            // Substitute '?' with '#' and '.'.
            count_arrangements(
                String::from("#") + &pattern[1..],
                counts.clone(),
                residual,
                cache,
            ) + count_arrangements(
                String::from(".") + &pattern[1..],
                counts.clone(),
                residual,
                cache,
            )
        }
        _ => unreachable!(),
    };

    *cache
        .entry((pattern, counts, residual))
        .or_insert(total_count)
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

        assert_eq!(run(input), 525152);
    }
}
