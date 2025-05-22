use std::{cmp::Ordering, str::FromStr};

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-7.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<u64> {
    let mut hands = input
        .lines()
        .map(Hand::from_str)
        .collect::<Result<Vec<_>>>()?;
    hands.sort();

    Ok(hands
        .into_iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) as u64 * hand.bid)
        .sum())
}

fn part_2(input: &str) -> Result<u64> {
    let mut hands = input
        .lines()
        .map(Hand::from_str)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|hand| hand.into_joker())
        .collect::<Vec<_>>();
    hands.sort();

    Ok(hands
        .into_iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) as u64 * hand.bid)
        .sum())
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
struct Card(u8);

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '2' => Ok(Card(2)),
            '3' => Ok(Card(3)),
            '4' => Ok(Card(4)),
            '5' => Ok(Card(5)),
            '6' => Ok(Card(6)),
            '7' => Ok(Card(7)),
            '8' => Ok(Card(8)),
            '9' => Ok(Card(9)),
            'T' => Ok(Card(10)),
            'J' => Ok(Card(11)),
            'Q' => Ok(Card(12)),
            'K' => Ok(Card(13)),
            'A' => Ok(Card(14)),
            _ => Err(anyhow!("Invalid input: {}", value)),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Card {
    fn into_joker(self) -> Self {
        if self.0 == 11 { Self(1) } else { self }
    }

    fn is_joker(&self) -> bool {
        self.0 == 1
    }
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
    hand_type: HandType,
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((cards, bid)) = s.split_once(" ") else {
            return Err(anyhow!("Cannot split input into cards and bid: {}", s));
        };

        let mut cards_vec = cards
            .chars()
            .map(Card::try_from)
            .collect::<Result<Vec<_>>>()?;
        let mut cards = [Card::default(); 5];
        for (index, card) in cards_vec.iter().enumerate() {
            cards[index] = *card;
        }

        let bid = bid.parse()?;

        cards_vec.sort();
        let hand_type = match cards_vec.as_slice() {
            [a, b, c, d, e] if a == b && a == c && a == d && a == e => HandType::FiveOfAKind,
            [a, b, c, d, _] | [_, a, b, c, d] if a == b && a == c && a == d => {
                HandType::FourOfAKind
            }
            [a, b, c, d, e] if (a == b && a == c && d == e) || (a == b && c == d && c == e) => {
                HandType::FullHouse
            }
            [a, b, c, _, _] | [_, a, b, c, _] | [_, _, a, b, c] if a == b && a == c => {
                HandType::ThreeOfAKind
            }
            [a, b, c, d, _] | [a, b, _, c, d] | [_, a, b, c, d] if a == b && c == d => {
                HandType::TwoPair
            }
            [a, b, _, _, _] | [_, a, b, _, _] | [_, _, a, b, _] | [_, _, _, a, b] if a == b => {
                HandType::OnePair
            }
            _ => HandType::HighCard,
        };

        Ok(Self {
            cards,
            bid,
            hand_type,
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => (),
            ord => return ord,
        }

        match self.cards[0].cmp(&other.cards[0]) {
            Ordering::Equal => (),
            ord => return ord,
        }

        match self.cards[1].cmp(&other.cards[1]) {
            Ordering::Equal => (),
            ord => return ord,
        }

        match self.cards[2].cmp(&other.cards[2]) {
            Ordering::Equal => (),
            ord => return ord,
        }

        match self.cards[3].cmp(&other.cards[3]) {
            Ordering::Equal => (),
            ord => return ord,
        }

        self.cards[4].cmp(&other.cards[4])
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn into_joker(mut self) -> Self {
        self.cards
            .iter_mut()
            .for_each(|card| *card = card.into_joker());

        // Upgrade hand type.
        let jokers = self.cards.iter().filter(|card| card.is_joker()).count();
        self.hand_type = match (self.hand_type, jokers) {
            (HandType::FourOfAKind, 1 | 4) => HandType::FiveOfAKind,
            (HandType::FullHouse, 2 | 3) => HandType::FiveOfAKind,
            (HandType::ThreeOfAKind, 1 | 3) => HandType::FourOfAKind,
            (HandType::TwoPair, 1) => HandType::FullHouse,
            (HandType::TwoPair, 2) => HandType::FourOfAKind,
            (HandType::OnePair, 1 | 2) => HandType::ThreeOfAKind,
            (HandType::HighCard, 1) => HandType::OnePair,
            _ => self.hand_type,
        };

        self
    }
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 6440);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 5905);

        Ok(())
    }
}
