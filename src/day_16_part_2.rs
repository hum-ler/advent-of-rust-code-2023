use crate::{clean_lines, day_16_part_1::*};

pub fn run(input: &str) -> usize {
    let hash_map = parse_input(input);

    let mut configs = Vec::<Config>::new();
    let lines = clean_lines(input).collect::<Vec<&str>>();
    let width = lines[0].len();
    let height = lines.len();

    let x: u8 = width.try_into().unwrap();
    (1..height + 1).for_each(|y| {
        let y: u8 = y.try_into().unwrap();
        configs.push(((1, y), Beam::Right));
        configs.push(((x, y), Beam::Left));
    });

    let y: u8 = height.try_into().unwrap();
    (1..width + 1).for_each(|x| {
        let x: u8 = x.try_into().unwrap();
        configs.push(((x, 1), Beam::Down));
        configs.push(((x, y), Beam::Up));
    });

    configs
        .iter()
        .map(|config| start_beam(*config, &hash_map))
        .max()
        .unwrap()
}

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

        assert_eq!(run(input), 51);
    }
}
