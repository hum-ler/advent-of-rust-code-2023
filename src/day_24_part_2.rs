use std::{num::ParseIntError, str::FromStr};

use nalgebra::Vector3;

pub fn run(input: &str) -> i128 {
    // Solution from https://www.reddit.com/r/adventofcode/comments/18pnycy/2023_day_24_solutions/
    //
    // Let's be frank: I'd forgotten most of my linear algebra and vector spaces, and probably can't
    // come up with a solution without some intensive study. It's funny because quite possibly I'd
    // have found this fairly easy when I was doing 'A' levels.

    let hailstones = input
        .trim()
        .split_terminator("\n")
        .filter_map(|line| {
            let line = line.trim();
            if !line.is_empty() {
                Some(Hailstone::from_str(line))
            } else {
                None
            }
        })
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    // p1 = position_1 - position_0
    // v1 = velocity_1 - velocity_0
    // p2 = position_2 - position_0
    // v2 = velocity_2 - velocity_0

    let p1 = hailstones[1].pos - hailstones[0].pos;
    let v1 = hailstones[1].velocity - hailstones[0].velocity;
    let p2 = hailstones[2].pos - hailstones[0].pos;
    let v2 = hailstones[2].velocity - hailstones[0].velocity;

    // t1 = -((p1 x p2) * v2) / ((v1 x p2) * v2)

    let t1 = -p1.cross(&p2).dot(&v2) / v1.cross(&p2).dot(&v2);

    // t2 = -((p1 x p2) * v1) / ((p1 x v2) * v1)

    let t2 = -p1.cross(&p2).dot(&v1) / p1.cross(&v2).dot(&v1);

    // c1 = position_1 + t1 * velocity_1
    // c2 = position_2 + t2 * velocity_2

    // Multiplication of i128 is not implemented by Matrix.
    let c1 = hailstones[1].pos
        + Vector3::from([
            t1 * hailstones[1].velocity.x,
            t1 * hailstones[1].velocity.y,
            t1 * hailstones[1].velocity.z,
        ]);
    let c2 = hailstones[2].pos
        + Vector3::from([
            t2 * hailstones[2].velocity.x,
            t2 * hailstones[2].velocity.y,
            t2 * hailstones[2].velocity.z,
        ]);

    // v = (c2 - c1) / (t2 - t1)
    // p = c1 - t1 * v

    let v = (c2 - c1) / (t2 - t1);
    let p = c1 - Vector3::from([t1 * v.x, t1 * v.y, t1 * v.z]);

    p.x + p.y + p.z
}

#[derive(Debug)]
struct Hailstone {
    pos: Vector3<i128>,
    velocity: Vector3<i128>,
}

impl FromStr for Hailstone {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos_part, velocity_part) = s.split_once(" @ ").unwrap();

        let pos = Vector3::from_vec(
            pos_part
                .split_terminator(", ")
                .map(str::trim)
                .map(str::parse::<i128>)
                .collect::<Result<Vec<_>, Self::Err>>()?,
        );

        let velocity = Vector3::from_vec(
            velocity_part
                .split_terminator(", ")
                .map(str::trim)
                .map(str::parse::<i128>)
                .collect::<Result<Vec<_>, Self::Err>>()?,
        );

        Ok(Self { pos, velocity })
    }
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

        assert_eq!(run(input), 47);
    }
}
