pub fn run(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|pattern| {
            let vertical_reflection = pattern.find_vertical_reflection().unwrap_or_default();

            let mut horizontal_reflection = 0;
            if vertical_reflection == 0 {
                horizontal_reflection = pattern.find_horizontal_reflection().unwrap_or_default();
            }

            vertical_reflection + 100 * horizontal_reflection
        })
        .sum()
}

pub(crate) fn parse_input(input: &str) -> Vec<Pattern> {
    input
        .trim()
        .split("\n\n")
        .map(Pattern::from)
        .collect::<Vec<Pattern>>()
}

#[derive(Debug)]
pub(crate) struct Pattern {
    pub rows: Vec<Vec<char>>,
    pub columns: Vec<Vec<char>>,
}

impl Pattern {
    fn rows_into_columns(rows: &[Vec<char>]) -> Vec<Vec<char>> {
        let mut columns = Vec::<Vec<char>>::new();
        (0..rows[0].len()).for_each(|_| columns.push(Vec::<char>::new()));

        rows.iter().for_each(|row| {
            row.iter()
                .enumerate()
                .for_each(|(index, c)| columns[index].push(*c))
        });

        columns
    }

    fn find_reflection(input: &Vec<Vec<char>>) -> Option<usize> {
        // Assumption: input has 0 or 1 reflection.

        let length = input.len();

        for index in 1..length {
            // Figure out the lhs and rhs ranges to compare to each other.
            let (left, right) = match index {
                left if left <= length / 2 => (0..=index - 1, index..=index * 2 - 1),
                _odd_right if length % 2 == 0 => {
                    (2 * index - length - 1..=index - 1, index..=length - 1)
                }
                _even_right => (2 * index - length..=index - 1, index..=length - 1),
            };

            if Self::is_reflection(&input[left], &input[right]) {
                return Some(index);
            }
        }

        None
    }

    pub(crate) fn is_reflection(input: &[Vec<char>], other: &[Vec<char>]) -> bool {
        input
            .iter()
            .rev()
            .enumerate()
            .all(|(index, input)| Self::is_reflecting_pair(input, &other[index]))
    }

    fn is_reflecting_pair(input: &Vec<char>, other: &Vec<char>) -> bool {
        input == other
    }

    pub(crate) fn find_vertical_reflection(&self) -> Option<usize> {
        Self::find_reflection(&self.columns)
    }

    pub(crate) fn find_horizontal_reflection(&self) -> Option<usize> {
        Self::find_reflection(&self.rows)
    }
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        // Assumption: all lines are of the same length.

        let rows = value
            .split('\n')
            .map(str::trim)
            .map(|token| token.chars())
            .map(|chars| chars.collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let columns = Self::rows_into_columns(&rows);

        Pattern { rows, columns }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        ";

        assert_eq!(run(input), 405);
    }
}
