use std::{cmp::max, str::FromStr};

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-2.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter_map(|game| {
            if game.is_possible(12, 13, 14) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum())
}

fn part_2(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<_>>>()?
        .iter()
        .map(Game::power)
        .sum())
}

/// [red, green, blue]
type Draw = [u8; 3];

struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((id, draws)) = s.strip_prefix("Game ").and_then(|s| s.split_once(": ")) else {
            return Err(anyhow!("Cannot split input into id and draws: {}", s));
        };
        let id = id.parse()?;

        let draws = draws
            .split_terminator("; ")
            .map(|token| {
                let mut draw = [0; 3];

                for token in token.split_terminator(", ") {
                    match token {
                        s if s.ends_with("red") => {
                            draw[0] = s
                                .strip_suffix(" red")
                                .ok_or(anyhow!("Invalid red: {}", s))?
                                .parse()?;
                        }
                        s if s.ends_with("green") => {
                            draw[1] = s
                                .strip_suffix(" green")
                                .ok_or(anyhow!("Invalid green: {}", s))?
                                .parse()?;
                        }
                        s if s.ends_with("blue") => {
                            draw[2] = s
                                .strip_suffix(" blue")
                                .ok_or(anyhow!("Invalid blue: {}", s))?
                                .parse()?;
                        }
                        _ => return Err(anyhow!("Invalid cubes: {}", token)),
                    }
                }

                Ok(draw)
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { id, draws })
    }
}

impl Game {
    fn is_possible(&self, red: u8, green: u8, blue: u8) -> bool {
        self.draws
            .iter()
            .all(|draw| draw[0] <= red && draw[1] <= green && draw[2] <= blue)
    }

    fn power(&self) -> u32 {
        self.draws
            .iter()
            .fold([0; 3], |mut acc, draw| {
                acc[0] = max(acc[0], draw[0] as u32);
                acc[1] = max(acc[1], draw[1] as u32);
                acc[2] = max(acc[2], draw[2] as u32);

                acc
            })
            .into_iter()
            .product()
    }
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 8);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 2286);

        Ok(())
    }
}
