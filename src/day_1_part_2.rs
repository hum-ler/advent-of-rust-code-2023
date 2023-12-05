pub fn run(input: &str) -> u32 {
    input
        .lines()
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(|token| find_calibration_value(token))
        .sum::<u32>()
}

fn find_calibration_value(input: &str) -> u32 {
    let first_digit = find_first_digit(input).unwrap();
    let last_digit = find_last_digit(input).unwrap();

    first_digit * 10 + last_digit
}

fn digit_patterns() -> Vec<(&'static str, u32)> {
    vec![
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
}

fn find_first_digit(input: &str) -> Option<u32> {
    if input.len() == 0 {
        return None;
    }

    for (pattern, substitute) in digit_patterns() {
        if input.starts_with(pattern) {
            return Some(substitute);
        }
    }

    find_first_digit(&input[1..])
}

fn find_last_digit(input: &str) -> Option<u32> {
    if input.len() == 0 {
        return None;
    }

    for (pattern, substitute) in digit_patterns() {
        if input.ends_with(pattern) {
            return Some(substitute);
        }
    }

    find_last_digit(&input[..input.len() - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        ";

        assert_eq!(run(input), 281);
    }
}
