use crate::clean_lines;

pub fn run(input: &str) -> i64 {
    calculate_area(&parse_input(input))
}

fn calculate_area(vertices: &[Coords]) -> i64 {
    // Using the [shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula). This will only
    // get us part of the area because we are working in a grid and should take the perimeter into
    // consideration.
    // ![illustration](docs/day-18-part-2.png)

    let inner_area = vertices
        .windows(2)
        .map(|w| w[0].0 * w[1].1 - w[1].0 * w[0].1)
        .sum::<i64>()
        .abs()
        / 2;

    let outer_area = vertices
        .windows(2)
        .map(|w| (w[1].0 - w[0].0).abs() + (w[1].1 - w[0].1).abs())
        .sum::<i64>()
        / 2 + 1;

    inner_area + outer_area
}

fn parse_input(input: &str) -> Vec<Coords> {
    let mut vertices = vec![(0, 0)];

    let mut vertex = (0, 0);
    clean_lines(input).for_each(|s| {
        vertex = parse_instruction(s, vertex);
        vertices.push(vertex);
    });

    vertices
}

/// Parses an instruction line.
///
/// Returns the destination Coords based on the given instruction and source Coords.
fn parse_instruction(input: &str, src: Coords) -> Coords {
    let distance = i64::from_str_radix(&input[input.len() - 7..input.len() - 2], 16).unwrap();
    let direction = &input[input.len() - 2..input.len() - 1];

    match direction {
        "0" => (src.0 + distance, src.1),
        "1" => (src.0, src.1 + distance),
        "2" => (src.0 - distance, src.1),
        "3" => (src.0, src.1 - distance),
        _ => unreachable!(),
    }
}

type Coords = (i64, i64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
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

        assert_eq!(run(input), 952408144115);
    }
}
