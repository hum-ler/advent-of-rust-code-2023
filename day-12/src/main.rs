use std::{collections::HashMap, str::FromStr};

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-12.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<u64> {
    let mut cache = HashMap::new();
    Ok(input
        .lines()
        .map(Row::from_str)
        .collect::<Result<Vec<_>>>()?
        .iter()
        .map(|row| count_arrangements(&row.pattern, &row.record, &mut cache))
        .sum())
}

fn part_2(input: &str) -> Result<u64> {
    let mut rows = input
        .lines()
        .map(Row::from_str)
        .collect::<Result<Vec<_>>>()?;

    for row in rows.iter_mut() {
        row.unfold()?;
    }

    let mut cache = HashMap::new();
    Ok(rows
        .iter()
        .map(|row| count_arrangements(&row.pattern, &row.record, &mut cache))
        .sum())
}

struct Row {
    pattern: String,
    record: Vec<usize>,
}

impl FromStr for Row {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((pattern, record)) = s.split_once(" ") else {
            return Err(anyhow!("Cannot split input into pattern and record: {}", s));
        };
        if !pattern.chars().all(|c| "?#.".contains(c)) {
            return Err(anyhow!("Invalid char in pattern: {}", pattern));
        }

        let pattern = String::from(pattern);
        let record = record
            .split_terminator(",")
            .map(str::parse::<usize>)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { pattern, record })
    }
}

impl Row {
    fn unfold(&mut self) -> Result<()> {
        self.pattern.push('?');
        self.pattern = self.pattern.repeat(5);
        self.pattern = String::from(
            self.pattern
                .strip_suffix('?')
                .ok_or(anyhow!("Cannot strip trailing ?"))?,
        );

        self.record = self.record.repeat(5);

        Ok(())
    }
}

fn count_arrangements<'a>(
    pattern: &'a str,
    record: &'a [usize],
    cache: &mut HashMap<(&'a str, &'a [usize]), u64>,
) -> u64 {
    if cache.contains_key(&(pattern, record)) {
        return cache[&(pattern, record)];
    }

    if pattern.is_empty() {
        return *cache
            .entry((pattern, record))
            .or_insert(record.is_empty() as u64);
    }

    if record.is_empty() {
        return *cache
            .entry((pattern, record))
            .or_insert(all_operational_springs(pattern) as u64);
    }

    // pattern must have sufficient space to cover all damaged springs with at least 1 separator in
    // between each cluster.
    if pattern.len() < record.iter().sum::<usize>() + record.len() - 1 {
        return *cache.entry((pattern, record)).or_default();
    }

    let mut arrangements = 0;
    if record.len() > 1 && starts_with_damaged_springs_plus_separator(pattern, record[0]) {
        arrangements += count_arrangements(&pattern[record[0] + 1..], &record[1..], cache);
    }
    if record.len() == 1 && starts_with_damaged_springs(pattern, record[0]) {
        arrangements += count_arrangements(&pattern[record[0]..], &record[1..], cache);
    }
    if starts_with_separator(pattern) {
        arrangements += count_arrangements(&pattern[1..], record, cache);
    }

    *cache.entry((pattern, record)).or_insert(arrangements)
}

fn all_operational_springs(pattern: &str) -> bool {
    pattern.chars().all(|c| "?.".contains(c))
}

fn all_damaged_springs(pattern: &str) -> bool {
    pattern.chars().all(|c| "?#".contains(c))
}

fn starts_with_damaged_springs_plus_separator(pattern: &str, damaged_springs: usize) -> bool {
    starts_with_damaged_springs(pattern, damaged_springs)
        && starts_with_separator(&pattern[damaged_springs..])
}

fn starts_with_damaged_springs(pattern: &str, damaged_springs: usize) -> bool {
    all_damaged_springs(&pattern[..damaged_springs])
}

fn starts_with_separator(pattern: &str) -> bool {
    all_operational_springs(&pattern[0..1])
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 21);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 525152);

        Ok(())
    }
}
