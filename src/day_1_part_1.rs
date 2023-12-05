pub fn run(input: &str) -> u32 {
    input
        .lines()
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(|token| find_calibration_value(token))
        .sum::<u32>()
}

fn find_calibration_value(input: &str) -> u32 {
    let digits = input.matches(char::is_numeric).collect::<Vec<&str>>();

    // Should panic if digits is empty or parsing fails.
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
