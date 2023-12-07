use std::collections::HashSet;

use crate::clean_lines;

pub fn run(input: &str) -> u32 {
    clean_lines(input)
        .map(parse_line)
        .map(|card| card.count_points())
        .sum::<u32>()
}

pub(crate) struct Card {
    _id: u32,
    winning_numbers: HashSet<u32>,
    picks: HashSet<u32>,
}

impl Card {
    pub(crate) fn count_matching_numbers(&self) -> usize {
        self.winning_numbers.intersection(&self.picks).count()
    }

    fn count_points(&self) -> u32 {
        let matching_numbers = self.count_matching_numbers();

        if matching_numbers == 0 {
            return 0;
        }

        2u32.pow((matching_numbers - 1).try_into().unwrap())
    }
}

pub(crate) fn parse_line(input: &str) -> Card {
    let (card_part, numbers_part) = input.split_once(':').unwrap();

    let id = card_part
        .split(' ')
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .skip(1)
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let (winning_numbers_part, picks_part) = numbers_part.split_once('|').unwrap();
    let winning_numbers = parse_numbers(winning_numbers_part);
    let picks = parse_numbers(picks_part);

    Card {
        _id: id,
        winning_numbers,
        picks,
    }
}

fn parse_numbers(input: &str) -> HashSet<u32> {
    input
        .split(' ')
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(|token| token.parse::<u32>().unwrap())
        .collect::<HashSet<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";

        assert_eq!(run(input), 13);
    }
}
