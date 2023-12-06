// t: time
// d: distance
// b: button time
//
// The time t can be divided into 2 parts: t = b + (t - b)
// The objective is to beat the distance d:
//         b(t - b) > d
//     => -b^2 + tb > d
//
// Solve for the floor (f) of the smaller b:
//         b^2 - tb + d = 0.
//     =>             f = floor((t - sqrt(t^2 - 4d)) / 2)
//
// f is the left tail of the distribution that will fail.
// Accounting for both tails, the number of ways to beat record: t - 2f - 1

use crate::clean_lines;

pub(crate) fn run(input: &str) -> u64 {
    parse_input(input)
        .iter()
        .map(|(time, distance)| count_record_beaters(*time, *distance))
        .product()
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    let input_values = clean_lines(input)
        .map(parse_line)
        .collect::<Vec<Vec<u64>>>();

    if input_values.len() != 2 || input_values[0].len() != input_values[1].len() {
        panic!("Unexpected input or parsing error");
    }

    input_values[0]
        .clone()
        .into_iter()
        .zip(input_values[1].clone())
        .collect()
}

fn parse_line(input: &str) -> Vec<u64> {
    let (_, numbers_part) = input.split_once(':').unwrap();

    numbers_part
        .split(' ')
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect()
}

pub(crate) fn count_record_beaters(time: u64, distance: u64) -> u64 {
    // Convert to floating point for calculations.
    let time = time as f64;
    let distance = distance as f64;

    let f = ((time - (time.powi(2) - 4f64 * distance).sqrt()) / 2f64).floor();

    (time - 2f64 * f - 1f64) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> String {
        String::from(
            r"
                Time:      7  15   30
                Distance:  9  40  200
            ",
        )
    }

    #[test]
    fn run_example() {
        assert_eq!(run(&example_input()), 288);
    }

    #[test]
    fn check_parsing() {
        assert_eq!(
            parse_input(&example_input()),
            vec![(7, 9), (15, 40), (30, 200)]
        );
    }
}
