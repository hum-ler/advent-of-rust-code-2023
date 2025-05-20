use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-1.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<u32> {
    input
        .lines()
        .map(|line| {
            let Some(first_digit) = line.bytes().find(|byte| byte.is_ascii_digit()) else {
                return Err(anyhow!("Cannot find first digit"));
            };
            let Some(last_digit) = line.bytes().rfind(|byte| byte.is_ascii_digit()) else {
                return Err(anyhow!("Cannot find last digit"));
            };

            Ok((first_digit - b'0') as u32 * 10 + (last_digit - b'0') as u32)
        })
        .sum()
}

fn part_2(input: &str) -> Result<u32> {
    let find_digit_prefix = |s: &str| match s {
        s if s.starts_with("0") || s.starts_with("zero") => Some(0u32),
        s if s.starts_with("1") || s.starts_with("one") => Some(1),
        s if s.starts_with("2") || s.starts_with("two") => Some(2),
        s if s.starts_with("3") || s.starts_with("three") => Some(3),
        s if s.starts_with("4") || s.starts_with("four") => Some(4),
        s if s.starts_with("5") || s.starts_with("five") => Some(5),
        s if s.starts_with("6") || s.starts_with("six") => Some(6),
        s if s.starts_with("7") || s.starts_with("seven") => Some(7),
        s if s.starts_with("8") || s.starts_with("eight") => Some(8),
        s if s.starts_with("9") || s.starts_with("nine") => Some(9),
        _ => None,
    };

    input
        .lines()
        .map(|line| {
            let mut index = 0;
            let mut first_digit = None;
            while index < line.len() && first_digit.is_none() {
                first_digit = find_digit_prefix(&line[index..]);
                index += 1;
            }
            let first_digit = first_digit.ok_or(anyhow!("Cannot find first digit"))?;

            let mut index = line.len();
            let mut last_digit = None;
            while index > 0 && last_digit.is_none() {
                last_digit = find_digit_prefix(&line[index - 1..]);
                index -= 1;
            }
            let last_digit = last_digit.ok_or(anyhow!("Cannot find last digit"))?;

            Ok(first_digit * 10 + last_digit)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    #[test]
    fn example_1() -> Result<()> {
        let example = r"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

        assert_eq!(part_1(trim_newlines(example))?, 142);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        let example = r"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

        assert_eq!(part_2(trim_newlines(example))?, 281);

        Ok(())
    }
}
