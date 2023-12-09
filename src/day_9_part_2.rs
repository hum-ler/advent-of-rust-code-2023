use crate::{clean_lines, day_9_part_1::*};

pub fn run(input: &str) -> i64 {
    clean_lines(input)
        .map(parse_line)
        .map(wrap_vector)
        .map(expand_differences)
        .map(extrapolate)
        .sum()
}

fn extrapolate(input: Vec<Vec<i64>>) -> i64 {
    let mut first_elements = input
        .iter()
        .map(|vec| *vec.first().unwrap())
        .collect::<Vec<i64>>();

    first_elements.reverse();

    first_elements
        .into_iter()
        .reduce(|acc, element| element - acc)
        .unwrap()
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

        assert_eq!(run(input), 2);
    }
}
