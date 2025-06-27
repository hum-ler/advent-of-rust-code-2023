use std::{collections::HashSet, str::FromStr};

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-4.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .map(Card::from_str)
        .collect::<Result<Vec<_>>>()?
        .iter()
        .map(Card::points)
        .sum())
}

fn part_2(input: &str) -> Result<u32> {
    let cards = input
        .lines()
        .map(Card::from_str)
        .collect::<Result<Vec<_>>>()?;

    let mut card_count = vec![1; cards.len()];
    for card in cards {
        let copies = card_count[card.id - 1]; // id starts from 1

        for count in card_count.iter_mut().skip(card.id).take(card.overlap()) {
            *count += copies;
        }
    }

    Ok(card_count.into_iter().sum())
}

struct Card {
    id: usize,
    winning_numbers: HashSet<u8>,
    numbers: HashSet<u8>,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((id, rhs)) = s.strip_prefix("Card ").and_then(|s| s.split_once(": ")) else {
            return Err(anyhow!("Cannot split input to get ID: {}", s));
        };
        let id = id.trim().parse()?;

        let Some((winning_numbers, numbers)) = rhs.split_once(" | ") else {
            return Err(anyhow!(
                "Cannot split input into winning numbers and numbers: {}",
                rhs
            ));
        };
        let winning_numbers = winning_numbers
            .split_whitespace()
            .map(str::parse::<u8>)
            .collect::<Result<HashSet<_>, _>>()?;
        let numbers = numbers
            .split_whitespace()
            .map(str::parse::<u8>)
            .collect::<Result<HashSet<_>, _>>()?;

        Ok(Self {
            id,
            winning_numbers,
            numbers,
        })
    }
}

impl Card {
    fn overlap(&self) -> usize {
        self.winning_numbers.intersection(&self.numbers).count()
    }

    fn points(&self) -> u32 {
        let overlap = self.overlap() as u32;

        if overlap == 0 {
            overlap
        } else {
            2u32.pow(overlap - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 13);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 30);

        Ok(())
    }
}
