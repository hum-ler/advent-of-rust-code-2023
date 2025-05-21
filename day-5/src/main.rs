use std::{ops::Range, str::FromStr};

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-5.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<u64> {
    let (seeds, mappers) = parse_input_into_seeds_and_mappers(input)?;

    seeds
        .into_iter()
        .map(|seed| mappers.iter().fold(seed, |acc, mapper| mapper.map(acc)))
        .min()
        .ok_or(anyhow!("Cannot find min location"))
}

fn part_2(input: &str) -> Result<u64> {
    let (seeds, mappers) = parse_input_into_seeds_and_mappers(input)?;

    seeds
        .chunks(2)
        .map(|chunk| {
            #[allow(clippy::single_range_in_vec_init)]
            let mut ranges = vec![chunk[0]..chunk[0] + chunk[1]];

            for mapper in &mappers {
                ranges = mapper.map_ranges(ranges);
            }

            ranges
                .into_iter()
                .map(|range| range.start)
                .min()
                .ok_or(anyhow!(
                    "Cannot find min location for range: {}..{}",
                    chunk[0],
                    chunk[0] + chunk[1]
                ))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .min()
        .ok_or(anyhow!("Cannot find min location"))
}

struct Map {
    src: Range<u64>,
    dst: u64,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens = s.split_whitespace().collect::<Vec<_>>();
        if tokens.len() != 3 {
            return Err(anyhow!("Invalid input: {}", s));
        }

        let dst = tokens[0].parse()?;
        let src = tokens[1].parse()?;
        let range = tokens[2].parse::<u64>()?;

        Ok(Self {
            src: src..src + range,
            dst,
        })
    }
}

impl Map {
    fn map(&self, value: u64) -> Option<u64> {
        if self.src.contains(&value) {
            Some(self.dst + value - self.src.start)
        } else {
            None
        }
    }

    /// Maps the given ranges into the ranges of mapped values, and the ranges of unmapped values.
    fn map_ranges(&self, ranges: Vec<Range<u64>>) -> (Vec<Range<u64>>, Vec<Range<u64>>) {
        let mut values_that_map = Vec::new();
        let mut values_that_don_t_map = Vec::new();

        for range in ranges {
            match range {
                range if self.src.contains(&range.start) && !self.src.contains(&range.end) => {
                    // <--- src --->
                    //       <-- range -->

                    values_that_map.push(range.start..self.src.end);
                    values_that_don_t_map.push(self.src.end..range.end);
                }
                range if !self.src.contains(&range.start) && self.src.contains(&range.end) => {
                    //       <--- src --->
                    // <-- range -->

                    values_that_map.push(self.src.start..range.end);
                    values_that_don_t_map.push(range.start..self.src.start);
                }
                range if self.src.contains(&range.start) && self.src.contains(&range.end) => {
                    // <------ src ------>
                    //    <-- range -->

                    values_that_map.push(range);
                }
                range if range.contains(&self.src.start) && range.contains(&self.src.end) => {
                    //    <--- src --->
                    // <----- range ----->

                    values_that_map.push(self.src.clone());
                    values_that_don_t_map.push(range.start..self.src.start);
                    values_that_don_t_map.push(self.src.end..range.end);
                }
                _ => {
                    // <-- range --> <--- src --->  or  <--- src ---> <-- range -->
                    values_that_don_t_map.push(range);
                }
            }
        }

        // Do the actual translation.
        let values_mapped = values_that_map
            .into_iter()
            .map(|range| {
                let start = self.dst + range.start - self.src.start;
                let end = start + range.end - range.start;

                start..end
            })
            .collect::<Vec<_>>();

        (values_mapped, values_that_don_t_map)
    }
}

struct Mapper {
    maps: Vec<Map>,
}

impl FromStr for Mapper {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        if lines.is_empty() || !lines[0].ends_with("map:") {
            return Err(anyhow!("Invalid input: {}", s));
        }

        let maps = lines
            .into_iter()
            .skip(1)
            .map(Map::from_str)
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { maps })
    }
}

impl Mapper {
    fn map(&self, value: u64) -> u64 {
        if let Some(mapped_value) = self
            .maps
            .iter()
            .fold(None, |acc, map| acc.or(map.map(value)))
        {
            mapped_value
        } else {
            value
        }
    }

    fn map_ranges(&self, ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut ranges_mapped = Vec::new();
        let mut ranges_left_to_map = ranges;
        for map in &self.maps {
            let (values_mapped, values_unmapped) = map.map_ranges(ranges_left_to_map);

            ranges_mapped.extend(values_mapped);
            ranges_left_to_map = values_unmapped;
        }
        ranges_mapped.extend(ranges_left_to_map); // unmapped values stay the same

        ranges_mapped
    }
}

fn parse_input_into_seeds_and_mappers(input: &str) -> Result<(Vec<u64>, Vec<Mapper>)> {
    let tokens = input.split_terminator("\n\n").collect::<Vec<_>>();
    if tokens.len() != 8 {
        return Err(anyhow!("Invalid input: {}", input));
    }

    let seeds = tokens[0]
        .strip_prefix("seeds: ")
        .ok_or(anyhow!("Cannot split input into seeds: {}", tokens[0]))?
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()?;

    let mappers = tokens
        .into_iter()
        .skip(1)
        .map(Mapper::from_str)
        .collect::<Result<Vec<_>>>()?;

    Ok((seeds, mappers))
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
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

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 35);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 46);

        Ok(())
    }
}
