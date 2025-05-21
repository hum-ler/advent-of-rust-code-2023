use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-6.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<usize> {
    Ok(parse_input_into_records(input)?
        .iter()
        .map(count_winning_strategies)
        .product())
}

fn part_2(input: &str) -> Result<usize> {
    let record = parse_input_into_single_record(input)?;

    Ok(count_winning_strategies(&record))
}

/// (time, distance)
type Record = (u64, u64);

fn parse_input_into_records(input: &str) -> Result<Vec<Record>> {
    let lines = input.lines().collect::<Vec<_>>();
    if lines.len() != 2 {
        return Err(anyhow!("Invalid input: {}", input));
    }

    let times = lines[0]
        .strip_prefix("Time:")
        .ok_or(anyhow!("Invalid time: {}", lines[0]))?
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()?;
    let distances = lines[1]
        .strip_prefix("Distance:")
        .ok_or(anyhow!("Invalid distance: {}", lines[1]))?
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()?;
    if times.len() != distances.len() {
        return Err(anyhow!(
            "times and distances don't pair up: {:?}, {:?}",
            times,
            distances
        ));
    }

    Ok(times.into_iter().zip(distances).collect())
}

fn count_winning_strategies(record: &Record) -> usize {
    let &(time, distance) = record;

    (1..time)
        .filter(|speed| speed * (time - speed) > distance)
        .count()
}

fn parse_input_into_single_record(input: &str) -> Result<Record> {
    let lines = input.lines().collect::<Vec<_>>();
    if lines.len() != 2 {
        return Err(anyhow!("Invalid input: {}", input));
    }

    let time = lines[0]
        .strip_prefix("Time:")
        .ok_or(anyhow!("Invalid time: {}", lines[0]))?
        .replace(" ", "")
        .parse()?;
    let distance = lines[1]
        .strip_prefix("Distance:")
        .ok_or(anyhow!("Invalid distance: {}", lines[1]))?
        .replace(" ", "")
        .parse()?;

    Ok((time, distance))
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 288);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 71503);

        Ok(())
    }
}
