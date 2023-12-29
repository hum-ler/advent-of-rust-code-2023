// The position of a hailstone at time t_n can be described as: p_0 + t_n * v
// =>                                   x = p_0x + t_n * vx,
//                                      y = p_0y + t_n * vy
// =>                                   x = p_0x + ((y - p_0y) * vx) / vy
// =>                        vy(x - p_0x) = vx(y - p_0y)
// => vy(x) - vy(p_0x) - vx(y) + vx(p_0y) = 0
// =>                       vy(x) - vx(y) = vy(p_0x) - vx(p_0y)

use std::{ops::Range, str::FromStr};

use itertools::Itertools;
use nalgebra::{matrix, Vector4};

use crate::clean_lines;

pub fn run(input: &str) -> usize {
    run_test_area(input, 200000000000000.0..400000000000000.0)
}

fn run_test_area(input: &str, test_area: Range<f64>) -> usize {
    let hailstones = clean_lines(input)
        .map(parse_line)
        .collect::<Vec<Hailstone>>();

    hailstones
        .iter()
        .combinations(2)
        .filter(|h| intersect(h[0], h[1], &test_area))
        .count()
}

fn intersect(h1: &Hailstone, h2: &Hailstone, test_area: &Range<f64>) -> bool {
    let a = matrix![h1.w, -h1.z;
                    h2.w, -h2.z];
    let b = matrix![h1.w * h1.x - h1.z * h1.y;
                    h2.w * h2.x - h2.z * h2.y];

    if let Some(x) = a.lu().solve(&b) {
        let t_n1 = (x.x - h1.x) / h1.z;
        let t_n2 = (x.x - h2.x) / h2.z;

        return t_n1 > 0.0 && t_n2 > 0.0 && test_area.contains(&x.x) && test_area.contains(&x.y);
    }

    false
}

fn parse_line(input: &str) -> Hailstone {
    let (p_0_part, v_part) = input.split_once(" @ ").unwrap();

    let mut components = p_0_part
        .split(", ")
        .take(2)
        .map(str::trim)
        .map(f64::from_str)
        .map(Result::unwrap)
        .collect::<Vec<f64>>();
    let v_components = v_part
        .split(", ")
        .take(2)
        .map(str::trim)
        .map(f64::from_str)
        .map(Result::unwrap)
        .collect::<Vec<f64>>();
    components.extend(v_components);

    Vector4::from_row_slice(&components[..])
}

type Hailstone = Vector4<f64>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
            19, 13, 30 @ -2,  1, -2
            18, 19, 22 @ -1, -1, -2
            20, 25, 34 @ -2, -2, -4
            12, 31, 28 @ -1, -2, -1
            20, 19, 15 @  1, -5, -3
        ";

        assert_eq!(run_test_area(input, 7.0..27.0), 2);
    }
}
