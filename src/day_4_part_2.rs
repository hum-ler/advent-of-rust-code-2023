use super::day_4_part_1::*;

pub fn run(input: &str) -> usize {
    let cards = input
        .lines()
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(parse_line)
        .collect::<Vec<Card>>();

    let mut card_counter = vec![1; cards.len()];

    for (index, card) in cards.iter().enumerate() {
        let rows_impacted = index + 1..=index + card.count_matching_numbers();
        for row in rows_impacted {
            card_counter[row] += card_counter[index];
        }
    }

    card_counter.iter().sum::<usize>()
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

        assert_eq!(run(input), 30);
    }
}
