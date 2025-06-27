use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-20.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<u64> {
    let mut modules = parse_input_into_modules(input)?;

    let mut log = [0; 2];
    for _ in 0..1000 {
        button_pulse(&mut modules, &mut log);
    }

    Ok(log.into_iter().product())
}

fn part_2(input: &str) -> Result<u64> {
    // rx has only 1 source, which turns out to be a conjunction with 4 sources. Assuming the latter
    // 4 sources signal high at different periods, we need to find the lcm at which they coincide.

    let mut modules = parse_input_into_modules(input)?;

    let Some((track_dst, mut track_src)) = modules
        .iter()
        .find(|(_, module)| {
            if let Module::Conjunction { dst, .. } = module {
                dst.contains(&"rx")
            } else {
                false
            }
        })
        .map(|(id, module)| {
            let track_src = if let Module::Conjunction { src, .. } = module {
                src.keys().copied().collect::<HashSet<_>>()
            } else {
                HashSet::new()
            };
            let track_dst = *id;

            (track_dst, track_src)
        })
    else {
        return Err(anyhow!("Cannot find source of rx"));
    };
    let periods = track_high_signals(&mut modules, track_dst, &mut track_src);

    Ok(periods.into_iter().fold(1, lcm))
}

/// (from, to, value)
/// where for the value:
/// - Low pulse = false
/// - High pulse = true
type Pulse<'a> = (&'a str, &'a str, bool);

#[derive(Clone, Eq, PartialEq)]
enum Module<'a> {
    FlipFlop {
        on: bool,
        dst: Vec<&'a str>,
    },
    Conjunction {
        src: HashMap<&'a str, bool>,
        dst: Vec<&'a str>,
    },
    Broadcaster {
        dst: Vec<&'a str>,
    },
    Output,
}

impl<'a> Module<'a> {
    fn pulse(&mut self, pulse: Pulse<'a>) -> Vec<Pulse<'a>> {
        let (from, to, value) = pulse;

        match self {
            Module::FlipFlop { on, dst } => {
                if !value {
                    *on = !*on;

                    dst.iter().map(|dst| (to, *dst, *on)).collect()
                } else {
                    Vec::new()
                }
            }
            Module::Conjunction { src, dst } => {
                src.entry(from).and_modify(|state| *state = value);

                let value = !src.values().all(|state| *state);
                dst.iter().map(|dst| (to, *dst, value)).collect()
            }
            Module::Broadcaster { dst } => dst.iter().map(|dst| (to, *dst, value)).collect(),
            Module::Output => Vec::new(),
        }
    }
}

fn parse_input_into_modules(input: &str) -> Result<HashMap<&str, Module>> {
    // Scan through once to get all conjunctions ready to receive src.
    let mut modules = input
        .lines()
        .map(|line| {
            let Some((id, _)) = line.split_once(" -> ") else {
                return Err(anyhow!("Cannot split input to get ID: {}", line));
            };

            match id {
                "broadcaster" => Ok((
                    id,
                    Module::Broadcaster {
                        dst: Default::default(),
                    },
                )),
                s if s.starts_with("%") => Ok((
                    &id[1..],
                    Module::FlipFlop {
                        on: Default::default(),
                        dst: Default::default(),
                    },
                )),
                s if s.starts_with("&") => Ok((
                    &id[1..],
                    Module::Conjunction {
                        src: Default::default(),
                        dst: Default::default(),
                    },
                )),
                _ => Err(anyhow!("Invalid module ID: {}", id)),
            }
        })
        .collect::<Result<HashMap<_, _>>>()?;
    modules.entry("output").or_insert(Module::Output);

    for line in input.lines() {
        let Some((id, destinations)) = line.split_once(" -> ") else {
            return Err(anyhow!(
                "Cannot split input into ID and destinations: {}",
                line
            ));
        };

        // Set all module dst.
        let destinations = destinations.split_terminator(", ").collect::<Vec<_>>();
        let id = match id {
            "broadcaster" => {
                if let Some(Module::Broadcaster { dst }) = modules.get_mut(id) {
                    *dst = destinations.clone();
                }

                id
            }
            "output" => id,
            s if s.starts_with("%") => {
                if let Some(Module::FlipFlop { dst, .. }) = modules.get_mut(&id[1..]) {
                    *dst = destinations.clone();
                }

                &id[1..]
            }
            s if s.starts_with("&") => {
                if let Some(Module::Conjunction { dst, .. }) = modules.get_mut(&id[1..]) {
                    *dst = destinations.clone();
                }

                &id[1..]
            }
            _ => return Err(anyhow!("Invalid module ID: {}", id)),
        };

        // Set conjuction src.
        for destination in &destinations {
            if let Some(Module::Conjunction { src, .. }) = modules.get_mut(destination) {
                src.entry(id).or_default();
            }
        }
    }

    Ok(modules)
}

fn button_pulse(modules: &mut HashMap<&str, Module>, log: &mut [u64; 2]) {
    let mut pulses = VecDeque::from([("button", "broadcaster", false)]);
    while let Some(pulse) = pulses.pop_front() {
        let (_, dst, value) = pulse;

        log[value as usize] += 1;

        if let Some(module) = modules.get_mut(dst) {
            pulses.extend(module.pulse(pulse));
        }
    }
}

/// Finds the number of button presses where each [Module] from track_src first send a high pulse to
/// track_dst.
///
/// Returns the set of button presses in no particular order, one for each [Module] in track_src.
fn track_high_signals(
    modules: &mut HashMap<&str, Module>,
    track_dst: &str,
    track_src: &mut HashSet<&str>,
) -> HashSet<u64> {
    let mut periods = HashSet::new();
    let mut button_presses = 0;
    while !track_src.is_empty() {
        button_presses += 1;

        let mut pulses = VecDeque::from([("button", "broadcaster", false)]);
        while let Some(pulse) = pulses.pop_front() {
            let (src, dst, value) = pulse;

            if value && dst == track_dst && track_src.contains(src) {
                periods.insert(button_presses);
                track_src.remove(src);
            }

            if let Some(module) = modules.get_mut(dst) {
                pulses.extend(module.pulse(pulse));
            }
        }
    }

    periods
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    #[test]
    fn example_1a() -> Result<()> {
        let example = r"
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

        assert_eq!(part_1(trim_newlines(example))?, 32000000);

        Ok(())
    }

    #[test]
    fn example_1b() -> Result<()> {
        let example = r"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";

        assert_eq!(part_1(trim_newlines(example))?, 11687500);

        Ok(())
    }
}
