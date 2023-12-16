// Pad the input grid with a boundary ('#' tiles). A beam terminates under 3 conditions:
// 1. Beam is going through a tile that has previously been reached from the same direction.
// 2. Beam is getting split in a splitter tile that has previously been reached from the same- or
//    the direct opposite directions.
// 3. Beam is hitting a boundary tile.

use std::collections::{HashMap, VecDeque};

use crate::clean_lines;

pub fn run(input: &str) -> usize {
    start_beam(((1, 1), Beam::Right), &parse_input(input))
}

pub(crate) fn start_beam(config: Config, hash_map: &HashMap<Coords, Tile>) -> usize {
    let mut hash_map = hash_map.clone();

    let mut beam_queue = VecDeque::<Config>::new();
    beam_queue.push_back(config);

    while !beam_queue.is_empty() {
        let mut turn = beam_queue.pop_front();

        while let Some((coords, beam)) = turn {
            let mut tile = *hash_map.get(&coords).unwrap();
            let result = tile.traverse(beam);

            // Updating because we can't(?) get a mutable reference.
            hash_map.insert(tile.coords, tile);

            match result {
                (Some(next), branch) => {
                    if let Some(branch) = branch {
                        beam_queue.push_back(branch);
                    }

                    turn = Some(next);
                }
                (None, None) => turn = None,
                _ => panic!("Unexpected result {result:?} from traverse"),
            }
        }
    }

    hash_map.values().filter(|tile| tile.is_energized()).count()
}

pub(crate) fn parse_input(input: &str) -> HashMap<Coords, Tile> {
    let lines = clean_lines(input).collect::<Vec<&str>>();
    let width = lines[0].len();
    let height = lines.len();

    let mut hash_map = HashMap::new();

    (0..width + 2).for_each(|x| {
        let x: u8 = x.try_into().unwrap();

        hash_map.insert((x, 0), Tile::new((x, 0), '#'));
    });

    (1..height + 1).for_each(|y| {
        let mut line = String::from("#");
        line.push_str(lines[y - 1]);
        line.push('#');

        line.chars().enumerate().for_each(|(x, c)| {
            let x: u8 = x.try_into().unwrap();
            let y: u8 = y.try_into().unwrap();

            hash_map.insert((x, y), Tile::new((x, y), c));
        });
    });

    (0..width + 2).for_each(|x| {
        let x: u8 = x.try_into().unwrap();
        let y: u8 = (height + 1).try_into().unwrap();

        hash_map.insert((x, y), Tile::new((x, y), '#'));
    });

    hash_map
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Beam {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Tile {
    coords: Coords,
    label: char,
    from_top: bool,
    from_right: bool,
    from_bottom: bool,
    from_left: bool,
}

impl Tile {
    fn new(coords: Coords, label: char) -> Self {
        Self {
            coords,
            label,
            from_top: false,
            from_right: false,
            from_bottom: false,
            from_left: false,
        }
    }

    fn is_energized(&self) -> bool {
        if self.label == '#' {
            return false;
        }

        self.from_top || self.from_right || self.from_bottom || self.from_left
    }

    /// Moves a beam through this tile. Consumes the beam and returns 0, 1 or 2 resultant beams.
    fn traverse(&mut self, beam: Beam) -> (Option<Config>, Option<Config>) {
        if self.label == '#' || !self.record_traversal(&beam) {
            return (None, None);
        }

        match (self.label, beam) {
            ('.', Beam::Up) => (Some(((self.coords.0, self.coords.1 - 1), beam)), None),
            ('.', Beam::Right) => (Some(((self.coords.0 + 1, self.coords.1), beam)), None),
            ('.', Beam::Down) => (Some(((self.coords.0, self.coords.1 + 1), beam)), None),
            ('.', Beam::Left) => (Some(((self.coords.0 - 1, self.coords.1), beam)), None),
            ('/', Beam::Up) => (
                Some(((self.coords.0 + 1, self.coords.1), Beam::Right)),
                None,
            ),
            ('/', Beam::Right) => (Some(((self.coords.0, self.coords.1 - 1), Beam::Up)), None),
            ('/', Beam::Down) => (Some(((self.coords.0 - 1, self.coords.1), Beam::Left)), None),
            ('/', Beam::Left) => (Some(((self.coords.0, self.coords.1 + 1), Beam::Down)), None),
            ('\\', Beam::Up) => (Some(((self.coords.0 - 1, self.coords.1), Beam::Left)), None),
            ('\\', Beam::Right) => (Some(((self.coords.0, self.coords.1 + 1), Beam::Down)), None),
            ('\\', Beam::Down) => (
                Some(((self.coords.0 + 1, self.coords.1), Beam::Right)),
                None,
            ),
            ('\\', Beam::Left) => (Some(((self.coords.0, self.coords.1 - 1), Beam::Up)), None),
            ('|', Beam::Up) => (Some(((self.coords.0, self.coords.1 - 1), beam)), None),
            ('|', Beam::Down) => (Some(((self.coords.0, self.coords.1 + 1), beam)), None),
            ('|', Beam::Left) | ('|', Beam::Right) => (
                Some(((self.coords.0, self.coords.1 - 1), Beam::Up)),
                Some(((self.coords.0, self.coords.1 + 1), Beam::Down)),
            ),
            ('-', Beam::Up) | ('-', Beam::Down) => (
                Some(((self.coords.0 - 1, self.coords.1), Beam::Left)),
                Some(((self.coords.0 + 1, self.coords.1), Beam::Right)),
            ),
            ('-', Beam::Left) => (Some(((self.coords.0 - 1, self.coords.1), beam)), None),
            ('-', Beam::Right) => (Some(((self.coords.0 + 1, self.coords.1), beam)), None),
            ('#', _) => panic!("Buggy shortcircuit of #"),
            _ => panic!("Unhandled traversal of {beam:?} through {self:?}"),
        }
    }

    /// Records the traversal of a beam through a tile, if the same direction has not been recorded
    /// before.
    ///
    /// Return true if the direction is recorded. False, if already recorded previously.
    fn record_traversal(&mut self, beam: &Beam) -> bool {
        match beam {
            Beam::Up => {
                if self.from_bottom {
                    return false;
                }
                self.from_bottom = true;

                // If we are splitting, mark direct opposite direction also.
                if self.label == '-' {
                    self.from_top = true;
                }
            }
            Beam::Right => {
                if self.from_left {
                    return false;
                }
                self.from_left = true;

                if self.label == '|' {
                    self.from_right = true;
                }
            }
            Beam::Down => {
                if self.from_top {
                    return false;
                }
                self.from_top = true;

                if self.label == '-' {
                    self.from_bottom = true;
                }
            }
            Beam::Left => {
                if self.from_right {
                    return false;
                }
                self.from_right = true;

                if self.label == '|' {
                    self.from_left = true;
                }
            }
        }

        true
    }
}

type Coords = (u8, u8);
pub(crate) type Config = (Coords, Beam);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
        ";

        assert_eq!(run(input), 46);
    }
}
