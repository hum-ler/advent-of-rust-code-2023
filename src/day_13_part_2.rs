use crate::day_13_part_1::*;

pub fn run(input: &str) -> usize {
    let patterns = parse_input(input);

    let original_reflections = patterns
        .iter()
        .map(|pattern| {
            let vertical_reflection = pattern.find_vertical_reflection();
            let horizontal_reflection = pattern.find_horizontal_reflection();

            if vertical_reflection.is_none() && horizontal_reflection.is_none() {
                panic!("Cannot find original reflection");
            }

            (vertical_reflection, horizontal_reflection)
        })
        .collect::<Vec<(Option<usize>, Option<usize>)>>();

    patterns
        .iter()
        .enumerate()
        .map(|(index, pattern)| {
            let (x, y) = pattern.dimensions();

            for x in 0..x {
                for y in 0..y {
                    let new_pattern = pattern.fix_smudge(x, y);

                    let vertical_reflections = new_pattern.find_all_vertical_reflections();
                    for reflection in vertical_reflections {
                        if reflection != original_reflections[index].0 {
                            return (reflection.unwrap(), 0);
                        }
                    }

                    let horizontal_reflections = new_pattern.find_all_horizontal_reflections();
                    for reflection in horizontal_reflections {
                        if reflection != original_reflections[index].1 {
                            return (0, reflection.unwrap());
                        }
                    }
                }
            }

            panic!("Cannot find another reflection for pattern #{}", index + 1);
        })
        .map(|(x, y)| x + 100 * y)
        .sum()
}

impl Pattern {
    fn dimensions(&self) -> (usize, usize) {
        (self.columns.len(), self.rows.len())
    }

    fn fix_smudge(&self, column: usize, row: usize) -> Self {
        let mut rows = self.rows.clone();
        let mut columns = self.columns.clone();

        match rows[row][column] {
            '.' => {
                rows[row][column] = '#';
                columns[column][row] = '#';
            }
            '#' => {
                rows[row][column] = '.';
                columns[column][row] = '.';
            }
            _ => panic!(
                "Unexpected content at ({column}, {row}): {}",
                rows[row][column]
            ),
        }

        Self { rows, columns }
    }

    fn find_all_vertical_reflections(&self) -> Vec<Option<usize>> {
        Self::find_all_reflections(&self.columns)
    }

    fn find_all_horizontal_reflections(&self) -> Vec<Option<usize>> {
        Self::find_all_reflections(&self.rows)
    }

    fn find_all_reflections(input: &Vec<Vec<char>>) -> Vec<Option<usize>> {
        // Note that the Option is only for convenience -- the Vec will either be empty, or be
        // containing only Somes, never Nones.

        let length = input.len();

        let mut reflections = vec![];

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
                reflections.push(Some(index));
            }
        }

        reflections
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

        assert_eq!(run(input), 400);
    }
}
