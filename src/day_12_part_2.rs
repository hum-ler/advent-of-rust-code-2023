use rayon::prelude::*;

use crate::{clean_lines, day_12_part_1::*};

pub fn run(input: &str) -> usize {
    let parsed_input = clean_lines(input)
        .map(parse_line)
        .collect::<Vec<(Vec<char>, Vec<usize>)>>();

    parsed_input
        .par_iter()
        .map(|(pattern, counts)| arrangements(pattern, counts))
        .sum()
}

fn parse_line(input: &str) -> (Vec<char>, Vec<usize>) {
    let (pattern_part, counts_part) = input.split_once(' ').unwrap();

    let pattern = pattern_part.chars().collect::<Vec<char>>();
    let mut pattern_extended = pattern.clone();
    pattern_extended.push('?');
    pattern_extended.extend(&pattern);
    pattern_extended.push('?');
    pattern_extended.extend(&pattern);
    pattern_extended.push('?');
    pattern_extended.extend(&pattern);
    pattern_extended.push('?');
    pattern_extended.extend(&pattern);

    let counts = counts_part
        .split(',')
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect::<Vec<usize>>();
    let mut counts_extended = counts.clone();
    counts_extended.extend(&counts);
    counts_extended.extend(&counts);
    counts_extended.extend(&counts);
    counts_extended.extend(&counts);

    (pattern_extended, counts_extended)
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
