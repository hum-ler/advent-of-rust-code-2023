use crate::{clean_lines, day_6_part_1::count_record_beaters};

pub(crate) fn run(input: &str) -> u64 {
    let (time, distance) = parse_input(input);

    count_record_beaters(time, distance)
}

fn parse_input(input: &str) -> (u64, u64) {
    let input_values = clean_lines(input).map(parse_line).collect::<Vec<u64>>();

    if input_values.len() != 2 {
        panic!("Unexpected input or parsing error");
    }

    (input_values[0], input_values[1])
}

fn parse_line(input: &str) -> u64 {
    let (_, numbers_part) = input.split_once(':').unwrap();

    numbers_part.replace(' ', "").parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
            Time:      7  15   30
            Distance:  9  40  200
        ";

        assert_eq!(run(input), 71503);
    }
}
