use super::day_3_part_1::*;

pub fn run(input: &str) -> u32 {
    let lines = input
        .lines()
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .collect::<Vec<&str>>();

    // Get the dimensions for calculating ranges later.
    let (width, height) = get_size(&lines);

    let (symbols, numbers) = input
        .lines()
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .enumerate()
        .map(|(index, token)| parse_line(index, token))
        .fold((vec![], vec![]), |mut acc, (symbols, numbers)| {
            acc.0.push(symbols);
            acc.1.push(numbers);
            acc
        });

    let mut acc = 0;
    for (row, symbol_vec) in symbols.iter().enumerate() {
        for symbol in symbol_vec {
            if !symbol.is_asterisk() {
                continue;
            }

            // Form a box around the symbol, so row +/- 1.
            let row_range = match row {
                0 => 0..=1,
                last_row if last_row == height - 1 => last_row - 1..=last_row,
                _ => row - 1..=row + 1,
            };

            let mut adjacent_part_numbers = vec![];
            for number_vec in &numbers[row_range] {
                for number in number_vec {
                    if symbol.is_adjacent_to(number, width) {
                        adjacent_part_numbers.push(number.clone());
                    }
                }
            }

            if adjacent_part_numbers.len() == 2 {
                acc += adjacent_part_numbers[0].value * adjacent_part_numbers[1].value;
            }
        }
    }
    acc
}

impl Symbol {
    fn is_asterisk(&self) -> bool {
        self.value == '*'
    }

    fn is_adjacent_to(&self, number: &Number, width: usize) -> bool {
        // Reuse the previous function by faking a symbols argument.
        is_part_number(number, &[vec![self.clone()]], width)
    }
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

        assert_eq!(run(input), 467835)
    }
}
