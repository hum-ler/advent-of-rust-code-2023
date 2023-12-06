use crate::day_2_part_1::*;

pub(crate) fn run(input: &str) -> u32 {
    parse_input(input)
        .iter()
        .map(Game::find_minimum_combo)
        .map(|combo| combo.power())
        .sum::<u32>()
}

impl Game {
    fn find_minimum_combo(&self) -> Combo {
        let mut minimum_combo = Combo::default();

        for combo in &self.combos {
            if combo.red > minimum_combo.red {
                minimum_combo.red = combo.red;
            }
            if combo.green > minimum_combo.green {
                minimum_combo.green = combo.green;
            }
            if combo.blue > minimum_combo.blue {
                minimum_combo.blue = combo.blue;
            }
        }

        minimum_combo
    }
}

impl Combo {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
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
        assert_eq!(run(&example_input()), 2286);
    }
}
