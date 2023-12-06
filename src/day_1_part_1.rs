use crate::clean_lines;

pub(crate) fn run(input: &str) -> u32 {
    clean_lines(input)
        .map(|token| find_calibration_value(token))
        .sum::<u32>()
}

fn find_calibration_value(input: &str) -> u32 {
    let digits = input.matches(char::is_numeric).collect::<Vec<&str>>();

    let first_digit = digits[0].parse::<u32>().unwrap();
    let last_digit = digits[digits.len() - 1].parse::<u32>().unwrap();

    first_digit * 10 + last_digit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        ";

        assert_eq!(run(input), 142);
    }
}
