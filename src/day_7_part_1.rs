use crate::clean_lines;

pub fn run(input: &str) -> u64 {
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
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Card {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
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

        assert_eq!(run(input), 6440);
    }

    #[test]
    fn check_card_order() {
        assert!(Card::Two < Card::Three);
        assert!(Card::Two < Card::Ace);

        assert!(Card::Three < Card::Four);
        assert!(Card::Three < Card::King);

        assert!(Card::King < Card::Ace);
    }

    #[test]
    fn check_sequence_order() {
        assert!(
            Sequence {
                cards: [
                    Card::Three,
                    Card::Three,
                    Card::Three,
                    Card::Three,
                    Card::Two
                ]
            } > Sequence {
                cards: [Card::Two, Card::Two, Card::Two, Card::Two, Card::Ace]
            }
        );

        assert!(
            Sequence {
                cards: [
                    Card::Seven,
                    Card::Seven,
                    Card::Eight,
                    Card::Eight,
                    Card::Eight
                ]
            } > Sequence {
                cards: [
                    Card::Seven,
                    Card::Seven,
                    Card::Seven,
                    Card::Eight,
                    Card::Eight
                ]
            }
        );
    }

    #[test]
    fn check_hand_order() {
        assert!(
            Hand::FiveOfAKind(Sequence {
                cards: [Card::Two; 5]
            }) > Hand::HighCard(Sequence {
                cards: [Card::Two, Card::Three, Card::Four, Card::Five, Card::Six]
            })
        );
        assert!(
            Hand::FiveOfAKind(Sequence {
                cards: [Card::Two; 5]
            }) > Hand::HighCard(Sequence {
                cards: [Card::Three, Card::Four, Card::Five, Card::Six, Card::Seven]
            })
        );

        assert!(
            Hand::FourOfAKind(Sequence {
                cards: [
                    Card::Three,
                    Card::Three,
                    Card::Three,
                    Card::Three,
                    Card::Two
                ]
            }) > Hand::FourOfAKind(Sequence {
                cards: [Card::Two, Card::Two, Card::Two, Card::Two, Card::Ace]
            })
        );

        assert!(
            Hand::FullHouse(Sequence {
                cards: [
                    Card::Seven,
                    Card::Seven,
                    Card::Eight,
                    Card::Eight,
                    Card::Eight
                ]
            }) > Hand::FullHouse(Sequence {
                cards: [
                    Card::Seven,
                    Card::Seven,
                    Card::Seven,
                    Card::Eight,
                    Card::Eight
                ]
            })
        );
    }

    #[test]
    fn check_hand_equal() {
        assert_ne!(
            Hand::FourOfAKind(Sequence {
                cards: [
                    Card::Three,
                    Card::Three,
                    Card::Three,
                    Card::Three,
                    Card::Two
                ]
            }),
            Hand::FourOfAKind(Sequence {
                cards: [Card::Two, Card::Two, Card::Two, Card::Two, Card::Ace]
            })
        );

        assert_ne!(
            Hand::FullHouse(Sequence {
                cards: [
                    Card::Seven,
                    Card::Seven,
                    Card::Eight,
                    Card::Eight,
                    Card::Eight
                ]
            }),
            Hand::FullHouse(Sequence {
                cards: [
                    Card::Seven,
                    Card::Seven,
                    Card::Seven,
                    Card::Eight,
                    Card::Eight
                ]
            })
        );
    }
}
