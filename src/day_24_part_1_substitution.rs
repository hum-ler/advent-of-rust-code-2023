// If point at t0 is (x0, y0) and delta x = dx, delta y = dy, equation of the line is:
//         m =  dy / dx
//    y - y0 = m(x - x0)
// =>      y = mx + y0 - mx0 where x > x0 if dx > 0,
//                                 x < x0 if dx < 0,
//                                 y > y0 if dy > 0,
//                                 y < y0 if dy < 0
//
// Equating 2 lines:
//    m0x + y0 - m0x0 = m1x + y1 - m1x1
// =>               x = (y1 - y0 + m0x0 - m1x1) / (m0 - m1)

use std::{ops::Range, str::FromStr};

use itertools::Itertools;

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
    let m1 = h1.m.1 / h1.m.0;
    let m2 = h2.m.1 / h2.m.0;

    if m1 == m2 { // float comparison!
        return false;
    }

    let x = (h1.p0.1 - h2.p0.1 + m2 * h2.p0.0 - m1 * h1.p0.0) / (m2 - m1);
    let y = m1 * x + h1.p0.1 - m1 * h1.p0.0;

    // Check the collision happens after t0.
    // We can check just x because the hailstone travels in a straight line.
    if (h1.m.0 > 0.0 && x < h1.p0.0)
        || (h1.m.0 < 0.0 && x > h1.p0.0)
        || (h2.m.0 > 0.0 && x < h2.p0.0)
        || (h2.m.0 < 0.0 && x > h2.p0.0)
    {
        return false;
    }

    test_area.contains(&x) && test_area.contains(&y)
}

fn parse_line(input: &str) -> Hailstone {
    let (p0_part, m_part) = input.split_once(" @ ").unwrap();

    let p0_components = p0_part
        .split(", ")
        .take(2)
        .map(str::trim)
        .map(f64::from_str)
        .map(Result::unwrap)
        .collect::<Vec<f64>>();
    let m_components = m_part
        .split(", ")
        .take(2)
        .map(str::trim)
        .map(f64::from_str)
        .map(Result::unwrap)
        .collect::<Vec<f64>>();

    Hailstone {
        p0: (p0_components[0], p0_components[1]),
        m: (m_components[0], m_components[1]),
    }
}

type Coords = (f64, f64);

struct Hailstone {
    p0: Coords,
    m: Coords,
}

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
