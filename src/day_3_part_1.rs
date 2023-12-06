use crate::clean_lines;

pub(crate) fn run(input: &str) -> u32 {
    let lines = clean_lines(input).collect::<Vec<&str>>();

    // Get the dimensions for calculating ranges later.
    let (width, height) = get_size(&lines);

    let (symbols, numbers) = clean_lines(input)
        .enumerate()
        .map(|(index, token)| parse_line(index, token))
        .fold((vec![], vec![]), |mut acc, (symbols, numbers)| {
            acc.0.push(symbols);
            acc.1.push(numbers);
            acc
        });

    let mut acc = 0;
    for (row, number_vec) in numbers.iter().enumerate() {
        for number in number_vec {
            // Form a box around the number, so row +/- 1, except for first or last row.
            let row_range = match row {
                0 => 0..=1,
                last_row if last_row == height - 1 => last_row - 1..=last_row,
                _ => row - 1..=row + 1,
            };

            if is_part_number(number, &symbols[row_range], width) {
                acc += number.value;
            }
        }
    }

    acc
}

#[derive(Clone, Debug)]
pub(crate) struct Location {
    _row: usize,
    column: usize,
    size: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct Number {
    location: Location,
    pub value: u32,
}

#[derive(Clone, Debug)]
pub(crate) struct Symbol {
    location: Location,
    pub value: char,
}

pub(crate) fn get_size(lines: &Vec<&str>) -> (usize, usize) {
    (lines.first().unwrap().len(), lines.len())
}

pub(crate) fn parse_line(row: usize, input: &str) -> (Vec<Symbol>, Vec<Number>) {
    let mut symbols = vec![];
    let mut numbers = vec![];

    (String::from(".") + input) // prepend a '.' to make window nicer to work with
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .enumerate()
        .for_each(|(column, pair)| {
            match pair[1] {
                '0'..='9' => {
                    let value = pair[1].to_digit(10).unwrap();

                    // Check if this is a continuation from the previous digit.
                    if pair[0].is_ascii_digit() {
                        // Pop the previous Number and push an updated one.
                        let last_number: Number = numbers.pop().unwrap();
                        numbers.push(Number {
                            location: Location {
                                _row: row,
                                column: last_number.location.column,
                                size: last_number.location.size + 1,
                            },
                            value: last_number.value * 10 + value,
                        });
                    } else {
                        numbers.push(Number {
                            location: Location {
                                _row: row,
                                column,
                                size: 1,
                            },
                            value,
                        });
                    }
                }
                '.' => (),
                _ => symbols.push(Symbol {
                    location: Location {
                        _row: row,
                        column,
                        size: 1,
                    },
                    value: pair[1],
                }),
            }
        });

    (symbols, numbers)
}

pub(crate) fn is_part_number(number: &Number, symbols: &[Vec<Symbol>], width: usize) -> bool {
    // Form a box around the number, so column +/- 1, except the first or last column.
    let column_range = match number.location.column {
        0 => 0..=number.location.size,
        last_column if last_column == width - 1 => last_column - 1..=last_column,
        column => column - 1..=column + number.location.size, // it is ok to go beyond width
    };

    for symbol_vec in symbols {
        for symbol in symbol_vec {
            if column_range.contains(&symbol.location.column) {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        ";

        assert_eq!(run(input), 4361);
    }
}
