use std::collections::HashSet;

use crate::{clean_lines, day_22_part_1::*};

pub fn run(input: &str) -> usize {
    let mut bricks = clean_lines(input).map(parse_line).collect::<Vec<Brick>>();
    bricks.sort_by(|a, b| a.position.2.cmp(&b.position.2));

    let bricks = settle(bricks);

    (0..bricks.len())
        .map(|b| {
            let mut acc = HashSet::new();
            acc.insert(b);
            cascading_fall(&mut acc, b, &bricks);
            acc.len() - 1
        })
        .sum()
}

fn cascading_fall(acc: &mut HashSet<usize>, id: usize, bricks: &[Brick]) {
    // Terminate when this brick is not supporting anything above.
    let brick = &bricks[id];
    if brick.supports.is_empty() {
        return;
    }

    // Terminate when this brick does not cause any brick above to fall.
    let falling_from_above = brick
        .clone()
        .supports
        .into_iter()
        .filter(|i| {
            let brick_above = &bricks[*i];
            brick_above.supported_by.is_subset(acc)
        })
        .collect::<Vec<usize>>();
    if falling_from_above.is_empty() {
        return;
    }

    // Otherwise, cascade.
    for i in &falling_from_above {
        acc.insert(*i);
    }
    for i in &falling_from_above {
        cascading_fall(acc, *i, bricks);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
            1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9
        ";

        assert_eq!(run(input), 7);
    }
}
