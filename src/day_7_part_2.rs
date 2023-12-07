use crate::clean_lines;

pub(crate) fn run(input: &str) -> u64 {
    let mut wagers = clean_lines(input).map(parse_line).collect::<Vec<Wager>>();
    wagers.sort();

    wagers
        .iter()
        .enumerate()
        .map(|(rank, wager)| (rank + 1) as u64 * wager.bid)
        .sum::<u64>()
}

fn parse_line(input: &str) -> Wager {
    let (sequence_part, bid_part) = input.split_once(' ').unwrap();

    let hand = Hand::from(Sequence::from(sequence_part));
    let bid = bid_part.parse::<u64>().unwrap();

    Wager { hand, bid }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Card {
        match value {
            'J' => Card::Joker,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Unexpected card value: {value}"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Hand {
    HighCard(Sequence),
    OnePair(Sequence),
    TwoPair(Sequence),
    ThreeOfAKind(Sequence),
    FullHouse(Sequence),
    FourOfAKind(Sequence),
    FiveOfAKind(Sequence),
}

impl From<Sequence> for Hand {
    fn from(value: Sequence) -> Self {
        value.classify()
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Sequence {
    cards: [Card; 5],
}

impl Sequence {
    fn classify(self) -> Hand {
        let counts = self.count_cards();

        // Pull out the Jokers.
        let jokers = counts[0];
        let mut counts = counts[1..].to_owned(); // shadow the original counts

        // Use Jokers to push up the highest count.
        if jokers > 0 {
            let max = counts.iter().max().unwrap();
            for (index, count) in counts.iter().enumerate() {
                if count == max {
                    counts[index] += jokers;
                    break;
                }
            }
        }

        let max = counts.iter().max().unwrap();
        match max {
            5 => Hand::FiveOfAKind(self),
            4 => Hand::FourOfAKind(self),
            3 if counts.iter().any(|count| *count == 2) => Hand::FullHouse(self),
            3 => Hand::ThreeOfAKind(self),
            2 if counts.iter().filter(|count| **count == 2).count() == 2 => Hand::TwoPair(self),
            2 => Hand::OnePair(self),
            _ => Hand::HighCard(self),
        }
    }

    fn count_cards(&self) -> [u8; 13] {
        let mut counts = [0; 13];

        self.cards
            .iter()
            .for_each(|card| counts[*card as usize] += 1); // using implicit discriminants

        counts
    }
}

impl From<&str> for Sequence {
    fn from(input: &str) -> Self {
        Self {
            cards: input
                .trim()
                .chars()
                .into_iter()
                .map(Card::from)
                .collect::<Vec<Card>>()
                .try_into()
                .unwrap(),
        }
    }
}

#[derive(Debug)]
struct Wager {
    hand: Hand,
    bid: u64,
}

impl PartialEq for Wager {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl PartialOrd for Wager {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

impl Eq for Wager {}

impl Ord for Wager {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        ";

        assert_eq!(run(input), 5905);
    }

    #[test]
    fn check_hand_order() {
        assert!(
            Hand::FourOfAKind(Sequence {
                cards: [Card::Ten, Card::Five, Card::Five, Card::Joker, Card::Five]
            }) < Hand::FourOfAKind(Sequence {
                cards: [
                    Card::Queen,
                    Card::Queen,
                    Card::Queen,
                    Card::Joker,
                    Card::Ace
                ]
            })
        );
        assert!(
            Hand::FourOfAKind(Sequence {
                cards: [
                    Card::Queen,
                    Card::Queen,
                    Card::Queen,
                    Card::Joker,
                    Card::Ace
                ]
            }) < Hand::FourOfAKind(Sequence {
                cards: [Card::King, Card::Ten, Card::Joker, Card::Joker, Card::Ten]
            })
        );
        assert!(
            Hand::FourOfAKind(Sequence {
                cards: [Card::Ten, Card::Five, Card::Five, Card::Joker, Card::Five]
            }) < Hand::FourOfAKind(Sequence {
                cards: [Card::King, Card::Ten, Card::Joker, Card::Joker, Card::Ten]
            })
        );
    }
}
