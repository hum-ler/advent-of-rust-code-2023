use std::collections::HashMap;

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-8.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<u64> {
    let (directions, left, right) = parse_input_into_directions_and_elements(input)?;

    // AAA always maps to ZZZ and no other ??Z.
    steps("AAA", &directions, &left, &right)
}

fn part_2(input: &str) -> Result<u64> {
    // Brute-force won't work. We need to determine the cycle len of each ??A element and find the
    // lcm of all the lens.
    //
    // For the input, the ??A element and corresponding ??Z element both map to the same pair of
    // next elements (even though the left and right sides may be swapped) and the cycle restarts
    // from ??Z.

    let (directions, left, right) = parse_input_into_directions_and_elements(input)?;

    Ok(left
        .keys()
        .filter(|element| element.ends_with("A"))
        .map(|element| steps(element, &directions, &left, &right))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .fold(1, lcm))
}

type ElementMap<'a> = HashMap<&'a str, &'a str>;

fn parse_input_into_directions_and_elements(
    input: &str,
) -> Result<(Vec<u8>, ElementMap, ElementMap)> {
    let Some((directions, elements)) = input.split_once("\n\n") else {
        return Err(anyhow!(
            "Cannot split input into directions and elements: {}",
            input
        ));
    };

    let directions = directions.bytes().collect::<Vec<_>>();

    let (left, right) = elements
        .lines()
        .map(|line| {
            let Some((from, to)) = line.strip_suffix(")").and_then(|s| s.split_once(" = (")) else {
                return Err(anyhow!("Cannot split input into from and to: {}", line));
            };

            let Some((left, right)) = to.split_once(", ") else {
                return Err(anyhow!("Cannot split input into left and right: {}", to));
            };

            Ok(((from, left), (from, right)))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .unzip::<_, _, HashMap<_, _>, HashMap<_, _>>();

    Ok((directions, left, right))
}

fn steps(
    start_element: &str,
    directions: &[u8],
    left: &ElementMap,
    right: &ElementMap,
) -> Result<u64> {
    let mut steps = 1;
    let mut element = start_element;
    for direction in directions.iter().cycle() {
        match direction {
            b'L' => {
                element = left
                    .get(element)
                    .ok_or(anyhow!("Cannot get left element: {}", element))?
            }
            b'R' => {
                element = right
                    .get(element)
                    .ok_or(anyhow!("Cannot get right element: {}", element))?
            }
            _ => return Err(anyhow!("Invalid direction: {}", direction)),
        }

        if element.ends_with("Z") {
            break;
        }

        steps += 1;
    }

    Ok(steps)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    #[test]
    fn example_1a() -> Result<()> {
        let example = r"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

        assert_eq!(part_1(trim_newlines(example))?, 2);

        Ok(())
    }

    #[test]
    fn example_1b() -> Result<()> {
        let example = r"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

        assert_eq!(part_1(trim_newlines(example))?, 6);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        let example = r"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

        assert_eq!(part_2(trim_newlines(example))?, 6);

        Ok(())
    }
}
