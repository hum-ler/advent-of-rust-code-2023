use std::{
    cmp::{Ordering, max},
    str::FromStr,
};

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-22.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<usize> {
    let mut bricks = input
        .lines()
        .map(Brick::from_str)
        .collect::<Result<Vec<_>>>()?;
    bricks.sort();
    settle_bricks(&mut bricks);
    bricks.sort();

    Ok((0..bricks.len())
        .filter(|index| {
            let mut snapshot = bricks.clone();
            snapshot.remove(*index);

            settle_bricks(&mut snapshot) == 0
        })
        .count())
}

fn part_2(input: &str) -> Result<usize> {
    let mut bricks = input
        .lines()
        .map(Brick::from_str)
        .collect::<Result<Vec<_>>>()?;
    bricks.sort();
    settle_bricks(&mut bricks);
    bricks.sort();

    Ok((0..bricks.len())
        .map(|index| {
            let mut snapshot = bricks.clone();
            snapshot.remove(index);

            settle_bricks(&mut snapshot)
        })
        .sum())
}

/// (x, y, z)
type Coord = (usize, usize, usize);

#[derive(Clone, Copy, Eq, PartialEq)]
struct Brick {
    low: Coord,
    high: Coord,
    axis: usize,
}

impl FromStr for Brick {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((low, high)) = s.split_once("~") else {
            return Err(anyhow!("Cannot split input into low and high: {}", s));
        };

        let &[x, y, z] = low.split_terminator(",").collect::<Vec<_>>().as_slice() else {
            return Err(anyhow!("Cannot split input into x, y, and z: {}", low));
        };
        let x = x.parse()?;
        let y = y.parse()?;
        let z = z.parse()?;
        let low = (x, y, z);

        let &[x, y, z] = high.split_terminator(",").collect::<Vec<_>>().as_slice() else {
            return Err(anyhow!("Cannot split input into x, y, and z: {}", high));
        };
        let x = x.parse()?;
        let y = y.parse()?;
        let z = z.parse()?;
        let high = (x, y, z);

        let axis = match (low, high) {
            ((low_x, low_y, low_z), (high_x, high_y, high_z))
                if low_x < high_x && low_y == high_y && low_z == high_z =>
            {
                0
            }
            ((low_x, low_y, low_z), (high_x, high_y, high_z))
                if low_x == high_x && low_y < high_y && low_z == high_z =>
            {
                1
            }
            ((low_x, low_y, low_z), (high_x, high_y, high_z))
                if low_x == high_x && low_y == high_y && low_z < high_z =>
            {
                2
            }
            ((low_x, low_y, low_z), (high_x, high_y, high_z))
                if low_x == high_x && low_y == high_y && low_z == high_z =>
            {
                2
            }
            _ => return Err(anyhow!("Invalid delta: {:?}, {:?}", low, high)),
        };

        Ok(Self { low, high, axis })
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.low.2.cmp(&other.low.2) {
            Ordering::Equal => (),
            ord => return ord,
        }

        match self.low.cmp(&other.low) {
            Ordering::Equal => (),
            ord => return ord,
        }

        self.high.cmp(&other.high)
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Brick {
    /// Adjusts the z pos of the [Brick].
    ///
    /// Returns true if the z pos is updated, false if unchanged.
    fn drop(&mut self, low_z: usize) -> bool {
        if low_z == self.low.2 {
            false
        } else {
            self.high.2 = low_z + (self.high.2 - self.low.2);
            self.low.2 = low_z;

            true
        }
    }
}

/// Lowers all [Brick]s as far as they can go.
///
/// Returns the num of [Brick]s that are lowered.
///
/// sorted_bricks must be sorted by increasing z pos.
fn settle_bricks(sorted_bricks: &mut [Brick]) -> usize {
    let mut bricks_dropped = vec![false; sorted_bricks.len()];

    let (max_x, max_y) = sorted_bricks.iter().fold((0, 0), |acc, brick| {
        (max(acc.0, brick.high.0), max(acc.1, brick.high.1))
    });

    loop {
        // [y][x]
        let mut depths = vec![vec![0; max_x + 1]; max_y + 1];
        let mut drop_during_iteration = false;

        for (index, brick) in sorted_bricks.iter_mut().enumerate() {
            if brick.drop(max_depth(&depths, brick)) {
                bricks_dropped[index] = true;
                drop_during_iteration = true;
            }

            update_depths(&mut depths, brick);
        }

        if !drop_during_iteration {
            break;
        }
    }

    bricks_dropped.into_iter().filter(|brick| *brick).count()
}

/// Checks the lowest depth that the given [Brick] can be placed.
fn max_depth(depths: &[Vec<usize>], brick: &Brick) -> usize {
    match brick.axis {
        0 => (brick.low.0..=brick.high.0).fold(0, |acc, x| max(acc, depths[brick.low.1][x] + 1)),
        1 => (brick.low.1..=brick.high.1).fold(0, |acc, y| max(acc, depths[y][brick.low.0] + 1)),
        _ => depths[brick.low.1][brick.low.0] + 1,
    }
}

/// Updates depths with the given [Brick]'s z pos.
fn update_depths(depths: &mut [Vec<usize>], brick: &Brick) {
    match brick.axis {
        0 => (brick.low.0..=brick.high.0).for_each(|x| depths[brick.low.1][x] = brick.low.2),
        1 => (brick.low.1..=brick.high.1).for_each(|y| depths[y][brick.low.0] = brick.low.2),
        _ => depths[brick.low.1][brick.low.0] = brick.high.2,
    }
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 5);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 7);

        Ok(())
    }
}
