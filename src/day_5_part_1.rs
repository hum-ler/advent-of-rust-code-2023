use std::ops::Range;

pub fn run(input: &str) -> i64 {
    let (seeds, map_chain) = parse_input(input);

    seeds
        .into_iter()
        .map(|seed| map_chain.map(Some(seed)).unwrap())
        .min()
        .unwrap()
}

/// A series of steps, which step consisting of a bunch of Maps to check.
#[derive(Debug, Default, PartialEq)]
pub(crate) struct MapChain<T> {
    chain: Vec<Vec<Map<T>>>,
}

impl MapChain<i64> {
    fn new() -> Self {
        MapChain::default()
    }

    fn push(&mut self, step: Vec<Map<i64>>) {
        self.chain.push(step);
    }

    /// Runs through all the steps in the chain.
    pub(crate) fn map(&self, item: Option<i64>) -> Option<i64> {
        let mut destination = item;

        for step in &self.chain {
            let step_result = step
                .iter()
                .filter(|map| map.contains(destination))
                .map(|map| map.translate(destination))
                .collect::<Vec<Option<i64>>>()
                .first()
                .unwrap_or(&None)
                .to_owned();

            // If no mapping for source is found, the destination remains the same as the source.
            if step_result.is_some() {
                destination = step_result;
            }
        }

        destination
    }
}

#[derive(Debug, PartialEq)]
struct Map<T> {
    source_range: Range<T>,

    /// The increment / decrement to translate from source to destination.
    translation: T,
}

impl Map<i64> {
    fn new(destination_range_start: i64, source_range_start: i64, range_length: i64) -> Self {
        Map {
            source_range: source_range_start..source_range_start + range_length,
            translation: destination_range_start - source_range_start,
        }
    }

    fn contains(&self, item: Option<i64>) -> bool {
        if item.is_none() {
            return false;
        }

        self.source_range.contains(&item.unwrap())
    }

    fn translate(&self, item: Option<i64>) -> Option<i64> {
        if self.contains(item) {
            return Some(item.unwrap() + self.translation);
        }

        None
    }
}

pub(crate) fn parse_input(input: &str) -> (Vec<i64>, MapChain<i64>) {
    let sections = input.split("\n\n").map(str::trim).collect::<Vec<&str>>();

    let seeds = parse_seeds(sections[0]);

    // By observation, in the data file, the sections / steps are sequential.
    let mut map_chain = MapChain::new();
    for section in &sections[1..] {
        map_chain.push(parse_step(section));
    }

    (seeds, map_chain)
}

fn parse_seeds(input: &str) -> Vec<i64> {
    input
        .split(' ')
        .skip(1)
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect::<Vec<i64>>()
}

fn parse_step(input: &str) -> Vec<Map<i64>> {
    input
        .lines()
        .skip(1)
        .map(parse_map)
        .collect::<Vec<Map<i64>>>()
}

fn parse_map(input: &str) -> Map<i64> {
    let triplets = input
        .split(' ')
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect::<Vec<i64>>();

    if triplets.len() != 3 {
        panic!(
            "Incorrect size {} for triplet from {}",
            triplets.len(),
            input
        );
    }

    Map::new(triplets[0], triplets[1], triplets[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> String {
        String::from(
            r"
                seeds: 79 14 55 13

                seed-to-soil map:
                50 98 2
                52 50 48

                soil-to-fertilizer map:
                0 15 37
                37 52 2
                39 0 15

                fertilizer-to-water map:
                49 53 8
                0 11 42
                42 0 7
                57 7 4

                water-to-light map:
                88 18 7
                18 25 70

                light-to-temperature map:
                45 77 23
                81 45 19
                68 64 13

                temperature-to-humidity map:
                0 69 1
                1 0 69

                humidity-to-location map:
                60 56 37
                56 93 4
            ",
        )
    }

    #[test]
    fn run_example() {
        assert_eq!(run(&example_input()), 35);
    }

    #[test]
    fn check_parsing() {
        let mut map_chain = MapChain::new();
        map_chain.push(vec![Map::new(50, 98, 2), Map::new(52, 50, 48)]);
        map_chain.push(vec![
            Map::new(0, 15, 37),
            Map::new(37, 52, 2),
            Map::new(39, 0, 15),
        ]);
        map_chain.push(vec![
            Map::new(49, 53, 8),
            Map::new(0, 11, 42),
            Map::new(42, 0, 7),
            Map::new(57, 7, 4),
        ]);
        map_chain.push(vec![Map::new(88, 18, 7), Map::new(18, 25, 70)]);
        map_chain.push(vec![
            Map::new(45, 77, 23),
            Map::new(81, 45, 19),
            Map::new(68, 64, 13),
        ]);
        map_chain.push(vec![Map::new(0, 69, 1), Map::new(1, 0, 69)]);
        map_chain.push(vec![Map::new(60, 56, 37), Map::new(56, 93, 4)]);

        let seeds = vec![79, 14, 55, 13];

        assert_eq!(parse_input(&example_input()), (seeds, map_chain));
    }
}
