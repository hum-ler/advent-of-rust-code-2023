pub fn run(input: &str) -> usize {
    dfs(convert_input_to_maze(input))
}

type Coord = (usize, usize);

fn convert_input_to_maze(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .split_terminator("\n")
        .filter_map(|line| {
            let line = line.trim();

            if line.is_empty() {
                None
            } else {
                Some(line.bytes().collect())
            }
        })
        .collect()
}

fn dfs(mut maze: Vec<Vec<u8>>) -> usize {
    let row_count = maze.len();
    let col_count = maze.first().map_or(0, Vec::len);

    assert!(row_count > 2);
    assert!(col_count > 3);

    let entrance: Coord = (0, 1);
    let goal: Coord = (row_count - 1, col_count - 2);

    let mut max_path_len = 0usize;
    let mut branch_stack: Vec<(Vec<Vec<u8>>, Coord)> = Vec::default();

    // It's probably a good idea to restructure this so that we can memoize the longest length from
    // branch exit to goal, so total_len = current_len + max(branch_lens).

    let mut node = entrance;
    loop {
        match traverse(node, &mut maze, goal) {
            TraverseResult::Branch(exits) => {
                for exit in &exits[1..] {
                    branch_stack.push((maze.clone(), *exit));
                }
                node = exits[0];
            }
            TraverseResult::DeadEnd => {
                if let Some((prev_maze, prev_exit)) = branch_stack.pop() {
                    maze = prev_maze;
                    node = prev_exit;
                } else {
                    // No more branches to explore.
                    break;
                }
            }
            TraverseResult::Goal(len) => {
                if len > max_path_len {
                    max_path_len = len;
                }

                if let Some((prev_maze, prev_exit)) = branch_stack.pop() {
                    maze = prev_maze;
                    node = prev_exit;
                } else {
                    // No more branches to explore.
                    break;
                }
            }
        }
    }

    max_path_len
}

enum TraverseResult {
    // Hit a dead end, no available exit. To backtrack to last junction.
    DeadEnd,

    // Hit a junction, multi-exit. To save state and continue traversal. All exits are provided.
    Branch(Vec<Coord>),

    // Hit the goal. The total path length is provided.
    Goal(usize),
}

fn traverse(node: Coord, maze: &mut [Vec<u8>], goal: Coord) -> TraverseResult {
    if node == goal {
        return TraverseResult::Goal(count_path_len(maze));
    }

    maze[node.0][node.1] = b'O';

    let exits = find_exits(node, maze);
    match exits.len() {
        0 => TraverseResult::DeadEnd,
        1 => traverse(exits[0], maze, goal),
        2 | 3 => TraverseResult::Branch(exits),
        _ => unreachable!(),
    }
}

fn find_exits(node: Coord, maze: &[Vec<u8>]) -> Vec<Coord> {
    let (row, col) = node;

    let mut exits: Vec<Coord> = Vec::default();

    // n
    if row > 0 && maze[row - 1][col] != b'#' && maze[row - 1][col] != b'O' {
        exits.push((row - 1, col));
    }

    // e
    if maze[row][col + 1] != b'#' && maze[row][col + 1] != b'O' {
        exits.push((row, col + 1));
    }

    // s
    if maze[row + 1][col] != b'#' && maze[row + 1][col] != b'O' {
        exits.push((row + 1, col));
    }

    // w
    if maze[row][col - 1] != b'#' && maze[row][col - 1] != b'O' {
        exits.push((row, col - 1));
    }

    exits
}

fn count_path_len(maze: &[Vec<u8>]) -> usize {
    maze.iter()
        .map(|row| row.iter().filter(|byte| **byte == b'O').count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
            #.#####################
            #.......#########...###
            #######.#########.#.###
            ###.....#.>.>.###.#.###
            ###v#####.#v#.###.#.###
            ###.>...#.#.#.....#...#
            ###v###.#.#.#########.#
            ###...#.#.#.......#...#
            #####.#.#.#######.#.###
            #.....#.#.#.......#...#
            #.#####.#.#.#########v#
            #.#...#...#...###...>.#
            #.#.#v#######v###.###v#
            #...#.>.#...>.>.#.###.#
            #####v#.#.###v#.#.###.#
            #.....#...#...#.#.#...#
            #.#########.###.#.#.###
            #...###...#...#...#.###
            ###.###.#.###v#####v###
            #...#...#.#.>.>.#.>.###
            #.###.###.#.###.#.#v###
            #.....###...###...#...#
            #####################.#
        ";

        assert_eq!(run(input), 154);
    }
}
