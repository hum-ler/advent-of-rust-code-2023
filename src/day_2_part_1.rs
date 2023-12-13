use crate::clean_lines;

pub fn run(input: &str) -> u32 {
    let test = Combo {
        red: 12,
        green: 13,
        blue: 14,
    };

    parse_input(input)
        .iter()
        .filter(|game| game.is_possible(&test))
        .fold(0, |acc, game| acc + game.id)
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct Game {
    pub id: u32,
    pub combos: Vec<Combo>,
}

impl Game {
    fn is_possible(&self, test: &Combo) -> bool {
        self.combos.iter().all(|combo| {
            combo.red <= test.red && combo.blue <= test.blue && combo.green <= test.green
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Combo {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

pub(crate) fn parse_input(input: &str) -> Vec<Game> {
    clean_lines(input).map(parse_game).collect::<Vec<Game>>()
}

fn parse_game(input: &str) -> Game {
    let (game_part, combos_part) = input.split_once(':').unwrap();

    let game_id = game_part
        .trim()
        .split(' ')
        .skip(1)
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let game_combos = combos_part
        .split(';')
        .map(parse_combo)
        .collect::<Vec<Combo>>();

    Game {
        id: game_id,
        combos: game_combos,
    }
}

fn parse_combo(input: &str) -> Combo {
    let mut combo = Combo::default();

    let combo_fragments = input.trim().split(", ");
    for fragment in combo_fragments {
        let (count_part, color_part) = fragment.split_once(' ').unwrap();

        let count = count_part.parse::<u32>().unwrap();
        match color_part {
            "red" => combo.red = count,
            "green" => combo.green = count,
            "blue" => combo.blue = count,
            _ => (),
        }
    }

    combo
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> String {
        String::from(
            r"
                Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
            ",
        )
    }

    #[test]
    fn run_example() {
        assert_eq!(run(&example_input()), 8);
    }

    #[test]
    fn check_parsing() {
        assert_eq!(
            parse_input(&example_input()),
            vec![
                Game {
                    id: 1,
                    combos: vec![
                        Combo {
                            red: 4,
                            green: 0,
                            blue: 3,
                        },
                        Combo {
                            red: 1,
                            green: 2,
                            blue: 6,
                        },
                        Combo {
                            red: 0,
                            green: 2,
                            blue: 0,
                        },
                    ],
                },
                Game {
                    id: 2,
                    combos: vec![
                        Combo {
                            red: 0,
                            green: 2,
                            blue: 1,
                        },
                        Combo {
                            red: 1,
                            green: 3,
                            blue: 4,
                        },
                        Combo {
                            red: 0,
                            green: 1,
                            blue: 1,
                        },
                    ],
                },
                Game {
                    id: 3,
                    combos: vec![
                        Combo {
                            red: 20,
                            green: 8,
                            blue: 6,
                        },
                        Combo {
                            red: 4,
                            green: 13,
                            blue: 5,
                        },
                        Combo {
                            red: 1,
                            green: 5,
                            blue: 0,
                        },
                    ],
                },
                Game {
                    id: 4,
                    combos: vec![
                        Combo {
                            red: 3,
                            green: 1,
                            blue: 6,
                        },
                        Combo {
                            red: 6,
                            green: 3,
                            blue: 0,
                        },
                        Combo {
                            red: 14,
                            green: 3,
                            blue: 15,
                        },
                    ],
                },
                Game {
                    id: 5,
                    combos: vec![
                        Combo {
                            red: 6,
                            green: 3,
                            blue: 1,
                        },
                        Combo {
                            red: 1,
                            green: 2,
                            blue: 2,
                        },
                    ],
                },
            ]
        );
    }
}
