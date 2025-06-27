use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-15.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<usize> {
    input.split_terminator(",").map(hash).sum()
}

fn part_2(input: &str) -> Result<usize> {
    let steps = input
        .split_terminator(",")
        .map(Step::from_str)
        .collect::<Result<Vec<_>>>()?;

    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];
    for step in steps {
        match step {
            Step::Replace {
                box_index,
                label,
                focal_len,
            } => {
                if let Some(pos) = boxes[box_index].iter().position(|lens| lens.0 == label) {
                    boxes[box_index][pos] = (label, focal_len);
                } else {
                    boxes[box_index].push((label, focal_len));
                }
            }
            Step::Remove { box_index, label } => {
                if let Some(pos) = boxes[box_index].iter().position(|lens| lens.0 == label) {
                    boxes[box_index].remove(pos);
                }
            }
        }
    }

    Ok(focusing_power(boxes))
}

fn hash(input: &str) -> Result<usize> {
    let mut curr_value = 0;

    for byte in input.bytes() {
        if !byte.is_ascii() {
            return Err(anyhow!("Invalid byte: {}", byte));
        }

        curr_value = ((curr_value + byte as usize) * 17) % 256;
    }

    Ok(curr_value)
}

enum Step<'a> {
    Replace {
        box_index: usize,
        label: &'a str,
        focal_len: u8,
    },
    Remove {
        box_index: usize,
        label: &'a str,
    },
}

impl<'a> Step<'a> {
    fn from_str(s: &'a str) -> Result<Self> {
        match s {
            s if s.contains('=') => {
                let Some((label, focal_len)) = s.split_once("=") else {
                    return Err(anyhow!(
                        "Cannot split input into label and focal len: {}",
                        s
                    ));
                };

                let box_index = hash(label)?;
                let focal_len = focal_len.parse()?;

                Ok(Self::Replace {
                    box_index,
                    label,
                    focal_len,
                })
            }
            s if s.ends_with('-') => {
                let label = s
                    .strip_suffix('-')
                    .ok_or(anyhow!("Cannot get label: {}", s))?;

                let box_index = hash(label)?;

                Ok(Self::Remove { box_index, label })
            }
            _ => Err(anyhow!("Invalid step: {}", s)),
        }
    }
}

type Lens<'a> = (&'a str, u8);

fn focusing_power(boxes: Vec<Vec<Lens>>) -> usize {
    boxes
        .into_iter()
        .enumerate()
        .flat_map(|(box_index, lenses)| {
            lenses
                .into_iter()
                .enumerate()
                .map(move |(lens_index, lens)| (1 + box_index) * (1 + lens_index) * lens.1 as usize)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE)?, 1320);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE)?, 145);

        Ok(())
    }
}
