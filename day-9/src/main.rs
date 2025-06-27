use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-9.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<i64> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse::<i64>)
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(extrapolate)
        .sum()
}

fn part_2(input: &str) -> Result<i64> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse::<i64>)
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(extrapolate_backwards)
        .sum()
}

fn extrapolate(mut numbers: Vec<i64>) -> Result<i64> {
    let mut last_numbers = vec![*numbers.last().ok_or(anyhow!("Cannot get last number"))?];
    while !numbers.iter().all(|number| *number == 0) {
        numbers = numbers
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();

        last_numbers.push(*numbers.last().ok_or(anyhow!("Cannot get last number"))?);
    }

    last_numbers
        .into_iter()
        .rev()
        .reduce(|acc, number| acc + number)
        .ok_or(anyhow!("Cannot extrapolate next number"))
}

fn extrapolate_backwards(mut numbers: Vec<i64>) -> Result<i64> {
    let mut first_numbers = vec![*numbers.first().ok_or(anyhow!("Cannot get first number"))?];
    while !numbers.iter().all(|number| *number == 0) {
        numbers = numbers
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();

        first_numbers.push(*numbers.first().ok_or(anyhow!("Cannot get first number"))?);
    }

    first_numbers
        .into_iter()
        .rev()
        .reduce(|acc, number| number - acc)
        .ok_or(anyhow!("Cannot extrapolate next number"))
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 114);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 2);

        Ok(())
    }
}
