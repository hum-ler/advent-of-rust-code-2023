use std::{ops::RangeInclusive, str::FromStr};

use anyhow::{Result, anyhow};
use itertools::Itertools;
use nalgebra::{Vector2, Vector3, matrix, vector};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-24.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<usize> {
    test_collisions_2d(input, 200000000000000.0..=400000000000000.0)
}

fn part_2(input: &str) -> Result<i128> {
    // Can't solve this. See
    // https://www.reddit.com/r/adventofcode/comments/18pnycy/2023_day_24_solutions/.
    //
    // Have to use i128 matrices to overcome floating point errors.

    let hailstones = input
        .lines()
        .map(HailstoneI128::from_str)
        .collect::<Result<Vec<_>>>()?;

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

    // Matrix does not impl scalar mul of i128.
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

    Ok(p.x + p.y + p.z)
}

#[derive(Clone, Copy)]
struct Hailstone {
    pos: Vector3<f64>,
    velocity: Vector3<f64>,
}

impl FromStr for Hailstone {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((pos, velocity)) = s.split_once(" @ ") else {
            return Err(anyhow!("Cannot split input into pos and velocity: {}", s));
        };

        let pos = Vector3::from_iterator(
            pos.split_terminator(",")
                .map(str::trim)
                .map(str::parse::<f64>)
                .collect::<Result<Vec<_>, _>>()?,
        );

        let velocity = Vector3::from_iterator(
            velocity
                .split_terminator(",")
                .map(str::trim)
                .map(str::parse::<f64>)
                .collect::<Result<Vec<_>, _>>()?,
        );

        Ok(Self { pos, velocity })
    }
}

struct HailstoneI128 {
    pos: Vector3<i128>,
    velocity: Vector3<i128>,
}

impl FromStr for HailstoneI128 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((pos, velocity)) = s.split_once(" @ ") else {
            return Err(anyhow!("Cannot split input into pos and velocity: {}", s));
        };

        let pos = Vector3::from_iterator(
            pos.split_terminator(",")
                .map(str::trim)
                .map(str::parse::<i128>)
                .collect::<Result<Vec<_>, _>>()?,
        );

        let velocity = Vector3::from_iterator(
            velocity
                .split_terminator(",")
                .map(str::trim)
                .map(str::parse::<i128>)
                .collect::<Result<Vec<_>, _>>()?,
        );

        Ok(Self { pos, velocity })
    }
}

fn test_collisions_2d(input: &str, range: RangeInclusive<f64>) -> Result<usize> {
    Ok(input
        .lines()
        .map(Hailstone::from_str)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .tuple_combinations()
        .filter(|(hailstone, other)| {
            if let Some(solution) = solve_2d(hailstone, other) {
                let x = hailstone.pos.x + solution[0] * hailstone.velocity.x;
                let y = hailstone.pos.y + solution[0] * hailstone.velocity.y;

                solution[0] >= 0.0 && solution[1] >= 0.0 && range.contains(&x) && range.contains(&y)
            } else {
                false
            }
        })
        .count())
}

fn solve_2d(hailstone: &Hailstone, other: &Hailstone) -> Option<Vector2<f64>> {
    //    - p_x_hail + t_hail * v_x_hail = p_x_other + t_other * v_x_other
    //    - p_y_hail + t_hail * v_y_hail = p_y_other + t_other * v_y_other
    // => - v_x_hail * t_hail - v_x_other * t_other = p_x_other - p_x_hail
    //    - v_y_hail * t_hail - v_y_other * t_other = p_y_other - p_y_hail
    // Solve for t_hail and t_other.

    let coefficients = matrix![
        hailstone.velocity.x, -other.velocity.x;
        hailstone.velocity.y, -other.velocity.y;
    ];
    let constants = vector![other.pos.x - hailstone.pos.x, other.pos.y - hailstone.pos.y];

    coefficients.lu().solve(&constants)
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(test_collisions_2d(trim_newlines(EXAMPLE), 7.0..=27.0)?, 2);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 47);

        Ok(())
    }
}
