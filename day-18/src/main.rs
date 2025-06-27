use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-18.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<usize> {
    // See https://en.wikipedia.org/wiki/Shoelace_formula.

    let dig_plan = input
        .lines()
        .map(Instruction::from_str)
        .collect::<Result<Vec<_>>>()?;

    let vertices = convert_to_vertices(&dig_plan)?;

    Ok(area(&vertices))
}

fn part_2(input: &str) -> Result<usize> {
    let dig_plan = input
        .lines()
        .map(Instruction::from_str)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|instruction| instruction.convert_from_colour())
        .collect::<Result<Vec<_>>>()?;

    let vertices = convert_to_vertices(&dig_plan)?;

    Ok(area(&vertices))
}

struct Instruction<'a> {
    direction: &'a str,
    distance: isize,
    colour: &'a str,
}

impl<'a> Instruction<'a> {
    fn from_str(s: &'a str) -> Result<Self> {
        let tokens = s.split_whitespace().collect::<Vec<_>>();
        let [direction, distance, colour] = tokens.as_slice() else {
            return Err(anyhow!(
                "Cannot split input into direction, distance and colour: {}",
                s
            ));
        };

        let distance = distance.parse()?;

        Ok(Self {
            direction,
            distance,
            colour,
        })
    }

    fn convert_from_colour(&self) -> Result<Self> {
        let direction = match &self.colour[7..8] {
            "0" => "R",
            "1" => "D",
            "2" => "L",
            "3" => "U",
            _ => return Err(anyhow!("Invalid direction: {}", &self.colour[7..8])),
        };

        let distance = isize::from_str_radix(&self.colour[2..7], 16)?;

        Ok(Self {
            direction,
            distance,
            colour: self.colour,
        })
    }
}

/// (row, col)
type Coord = (isize, isize);

fn convert_to_vertices(dig_plan: &[Instruction]) -> Result<Vec<Coord>> {
    let mut curr_vertex = (0, 0);
    let mut vertices = vec![curr_vertex];
    for instruction in dig_plan {
        match instruction.direction {
            "U" => {
                curr_vertex = (curr_vertex.0 - instruction.distance, curr_vertex.1);
            }
            "R" => {
                curr_vertex = (curr_vertex.0, curr_vertex.1 + instruction.distance);
            }
            "D" => {
                curr_vertex = (curr_vertex.0 + instruction.distance, curr_vertex.1);
            }
            "L" => {
                curr_vertex = (curr_vertex.0, curr_vertex.1 - instruction.distance);
            }
            _ => return Err(anyhow!("Invalid direction: {}", instruction.direction)),
        };

        vertices.push(curr_vertex);
    }

    if vertices.first() != vertices.last() {
        return Err(anyhow!("Not a closed polygon: {:?}", vertices));
    }

    Ok(vertices)
}

fn area(vertices: &[Coord]) -> usize {
    // As the coords are at the centre of each 1 x 1 sq unit, we also need to consider the 1/2-unit
    // border along the perimeter.

    // Use shoelace formula.
    let polygon_area = vertices
        .windows(2)
        .map(|window| window[0].0 * window[1].1 - window[1].0 * window[0].1)
        .sum::<isize>()
        .unsigned_abs()
        / 2;

    // Imagine the top-left corner of a sq (it equally applies to 90-deg polygons):
    //    a   b
    //      |---
    //    c | d
    // We only need to consider a, b, and c:
    // - If we sum up all the a, we get 1 unit for the whole polygon (4 corners for a sq).
    // - If we sum up all the edges and then divide by 2, we account for b and c (the 1/2-unit that
    //   is perpendicular to the perimeter).
    let border_area = vertices
        .windows(2)
        .map(|window| window[1].0.abs_diff(window[0].0) + window[1].1.abs_diff(window[0].1))
        .sum::<usize>()
        / 2
        + 1;

    polygon_area + border_area
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 62);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 952408144115);

        Ok(())
    }
}
