use clap::Parser;

mod day_1_part_1;
mod day_1_part_2;
mod day_2_part_1;
mod day_2_part_2;
mod day_3_part_1;
mod day_3_part_2;
mod day_4_part_1;
mod day_4_part_2;
mod day_5_part_1;
mod day_5_part_2;
mod day_6_part_1;
mod day_6_part_2;
mod day_7_part_1;
mod day_7_part_2;

#[derive(Parser)]
struct Args {
    /// Day number, 1 to 25.
    day: u8,

    /// Part 1 or 2.
    part: u8,
}

fn main() {
    let args = Args::parse();
    match (args.day, args.part) {
        (1, 1) => println!("{}", day_1_part_1::run(&input("input/day-1.txt"))),
        (1, 2) => println!("{}", day_1_part_2::run(&input("input/day-1.txt"))),
        (2, 1) => println!("{}", day_2_part_1::run(&input("input/day-2.txt"))),
        (2, 2) => println!("{}", day_2_part_2::run(&input("input/day-2.txt"))),
        (3, 1) => println!("{}", day_3_part_1::run(&input("input/day-3.txt"))),
        (3, 2) => println!("{}", day_3_part_2::run(&input("input/day-3.txt"))),
        (4, 1) => println!("{}", day_4_part_1::run(&input("input/day-4.txt"))),
        (4, 2) => println!("{}", day_4_part_2::run(&input("input/day-4.txt"))),
        (5, 1) => println!("{}", day_5_part_1::run(&input("input/day-5.txt"))),
        (5, 2) => println!("{}", day_5_part_2::run(&input("input/day-5.txt"))),
        (6, 1) => println!("{}", day_6_part_1::run(&input("input/day-6.txt"))),
        (6, 2) => println!("{}", day_6_part_2::run(&input("input/day-6.txt"))),
        (7, 1) => println!("{}", day_7_part_1::run(&input("input/day-7.txt"))),
        (7, 2) => println!("{}", day_7_part_2::run(&input("input/day-7.txt"))),
        _ => (),
    };
}

/// Wraps around [std::fs::read_to_string].
pub fn input<P>(path: P) -> String
where
    P: AsRef<std::path::Path>,
{
    std::fs::read_to_string(path).unwrap()
}

/// Splits input into lines.
///
/// Also trims, and removes empty lines.
pub fn clean_lines(input: &str) -> impl Iterator<Item = &str> {
    input
        .lines()
        .map(str::trim)
        .filter(|token| !token.is_empty())
}
