// This is yet another case where brute-forcing is not likely to work.
// Looking at the input file, there is only one module: th -> rx, and there are 4 modules:
// (xn, qn, xf, zl) -> th. So, we attempt to find when each will output a high pulse to th. When the
// 4 pulses collide (*assuming that these cycle perfectly*) at the lcm, we should have the answer.

use std::collections::{HashMap, VecDeque};

use crate::{day_20_part_1::*, day_8_part_2::lcm};

pub fn run(input: &str) -> u64 {
    let modules_at_start = parse_input(input);

    let mut modules = modules_at_start.clone();
    let mut xn = 1;
    while !button_press(
        &mut modules,
        Signal {
            predecessor: "xn",
            successor: "th",
            pulse: true,
        },
    ) {
        xn += 1;
    }

    let mut modules = modules_at_start.clone();
    let mut qn = 1;
    while !button_press(
        &mut modules,
        Signal {
            predecessor: "qn",
            successor: "th",
            pulse: true,
        },
    ) {
        qn += 1;
    }

    let mut modules = modules_at_start.clone();
    let mut xf = 1;
    while !button_press(
        &mut modules,
        Signal {
            predecessor: "xf",
            successor: "th",
            pulse: true,
        },
    ) {
        xf += 1;
    }

    let mut modules = modules_at_start.clone();
    let mut zl = 1;
    while !button_press(
        &mut modules,
        Signal {
            predecessor: "zl",
            successor: "th",
            pulse: true,
        },
    ) {
        zl += 1;
    }

    [xn, qn, xf, zl].into_iter().reduce(lcm).unwrap()
}

/// Sends the initial pulse from button to broadcaster.
///
/// Returns true if given condition has been met. Otherwise, false.
fn button_press<'a>(modules: &mut HashMap<&'a str, Module<'a>>, condition: Signal) -> bool {
    let mut condition_met = false;

    let mut signals = VecDeque::new();
    signals.push_back(Signal {
        predecessor: "button",
        successor: "broadcaster",
        pulse: false,
    });

    while !signals.is_empty() {
        let signal = signals.pop_front().unwrap();

        if signal == condition {
            condition_met = true;
        }

        signals.append(
            &mut modules
                .get(signal.successor)
                .unwrap()
                .clone()
                .process_pulse(signal, modules),
        );
    }

    condition_met
}
