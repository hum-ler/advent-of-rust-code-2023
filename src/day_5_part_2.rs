use crate::day_5_part_1::*;

use rayon::prelude::*;

use std::ops::Range;

pub fn run(input: &str) -> i64 {
    let (seeds, map_chain) = parse_input(input);

    expand_seeds(seeds)
        .into_par_iter()
        .map(|range| find_range_location_min(range, &map_chain))
        .min()
        .unwrap()
}

fn expand_seeds(seeds: Vec<i64>) -> Vec<Range<i64>> {
    if seeds.is_empty() || seeds.len() % 2 == 1 {
        panic!("Invalid seeds: {:?}", seeds);
    }

    let mut ranges = Vec::<Range<i64>>::with_capacity(seeds.len() / 2);
    for index in 0..seeds.len() / 2 {
        ranges.push(seeds[index * 2]..seeds[index * 2] + seeds[index * 2 + 1]);
    }

    ranges
}

fn find_range_location_min(range: Range<i64>, map_chain: &MapChain<i64>) -> i64 {
    range
        .into_par_iter()
        .map(|seed| map_chain.map(Some(seed)).unwrap())
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
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
        ";

        assert_eq!(run(input), 46)
    }
}
