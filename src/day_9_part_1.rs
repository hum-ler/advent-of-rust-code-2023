use crate::clean_lines;

pub fn run(input: &str) -> i64 {
    clean_lines(input)
        .map(parse_line)
        .map(wrap_vector)
        .map(expand_differences)
        .map(extrapolate)
        .sum()
}

pub(crate) fn parse_line(input: &str) -> Vec<i64> {
    input
        .split(' ')
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect::<Vec<i64>>()
}

pub(crate) fn wrap_vector(history: Vec<i64>) -> Vec<Vec<i64>> {
    vec![history]
}

/// Calculates and returns all the layers from the initial history.
pub(crate) fn expand_differences(input: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    if input.len() != 1 {
        panic!("Unexpected initial history {input:?}");
    }

    let mut expanded = vec![input[0].clone()];
    let mut layer = input[0].clone();
    while !is_all_zeroes(&layer) {
        let differences = calculate_differences(&layer);
        layer = differences.clone();
        expanded.push(differences);
    }

    expanded
}

/// Calculates a new layer from the previous one.
fn calculate_differences(input: &Vec<i64>) -> Vec<i64> {
    input
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<i64>>()
}

fn is_all_zeroes(input: &Vec<i64>) -> bool {
    input.iter().all(|value| *value == 0)
}

fn extrapolate(input: Vec<Vec<i64>>) -> i64 {
    let mut last_elements = input.iter().map(|vec| *vec.last().unwrap()).collect::<Vec<i64>>();

    last_elements.reverse();

    last_elements.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        ";

        assert_eq!(run(input), 114);
    }
}
