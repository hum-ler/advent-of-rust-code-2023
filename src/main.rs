use clap::Parser;

use advent_of_rust_code_2023::*;

#[derive(Parser)]
struct Args {
    /// Day number: 1 to 25.
    day: u8,

    /// Part number: 1 or 2.
    part: u8,
}

fn main() {
    let args = Args::parse();
    match (args.day, args.part) {
        (1, 1) => println!("{}", run_day_1_part_1(&input("input/day-1.txt"))),
        (1, 2) => println!("{}", run_day_1_part_2(&input("input/day-1.txt"))),
        (2, 1) => println!("{}", run_day_2_part_1(&input("input/day-2.txt"))),
        (2, 2) => println!("{}", run_day_2_part_2(&input("input/day-2.txt"))),
        (3, 1) => println!("{}", run_day_3_part_1(&input("input/day-3.txt"))),
        (3, 2) => println!("{}", run_day_3_part_2(&input("input/day-3.txt"))),
        (4, 1) => println!("{}", run_day_4_part_1(&input("input/day-4.txt"))),
        (4, 2) => println!("{}", run_day_4_part_2(&input("input/day-4.txt"))),
        (5, 1) => println!("{}", run_day_5_part_1(&input("input/day-5.txt"))),
        (5, 2) => println!("{}", run_day_5_part_2(&input("input/day-5.txt"))),
        (6, 1) => println!("{}", run_day_6_part_1(&input("input/day-6.txt"))),
        (6, 2) => println!("{}", run_day_6_part_2(&input("input/day-6.txt"))),
        (7, 1) => println!("{}", run_day_7_part_1(&input("input/day-7.txt"))),
        (7, 2) => println!("{}", run_day_7_part_2(&input("input/day-7.txt"))),
        (8, 1) => println!("{}", run_day_8_part_1(&input("input/day-8.txt"))),
        (8, 2) => println!("{}", run_day_8_part_2(&input("input/day-8.txt"))),
        (9, 1) => println!("{}", run_day_9_part_1(&input("input/day-9.txt"))),
        (9, 2) => println!("{}", run_day_9_part_2(&input("input/day-9.txt"))),
        (10, 1) => println!("{}", run_day_10_part_1(&input("input/day-10.txt"))),
        (10, 2) => println!("{}", run_day_10_part_2(&input("input/day-10.txt"))),
        (11, 1) => println!("{}", run_day_11_part_1(&input("input/day-11.txt"))),
        (11, 2) => println!("{}", run_day_11_part_2(&input("input/day-11.txt"))),
        (13, 1) => println!("{}", run_day_13_part_1(&input("input/day-13.txt"))),
        (13, 2) => println!("{}", run_day_13_part_2(&input("input/day-13.txt"))),
        (14, 1) => println!("{}", run_day_14_part_1(&input("input/day-14.txt"))),
        (14, 2) => println!("{}", run_day_14_part_2(&input("input/day-14.txt"))),
        (15, 1) => println!("{}", run_day_15_part_1(&input("input/day-15.txt"))),
        (15, 2) => println!("{}", run_day_15_part_2(&input("input/day-15.txt"))),
        (16, 1) => println!("{}", run_day_16_part_1(&input("input/day-16.txt"))),
        (16, 2) => println!("{}", run_day_16_part_2(&input("input/day-16.txt"))),
        (17, 1) => println!("{}", run_day_17_part_1(&input("input/day-17.txt"))),
        (17, 2) => println!("{}", run_day_17_part_2(&input("input/day-17.txt"))),
        (18, 1) => println!("{}", run_day_18_part_1(&input("input/day-18.txt"))),
        (18, 2) => println!("{}", run_day_18_part_2(&input("input/day-18.txt"))),
        (19, 1) => println!("{}", run_day_19_part_1(&input("input/day-19.txt"))),
        (19, 2) => println!("{}", run_day_19_part_2(&input("input/day-19.txt"))),
        (20, 1) => println!("{}", run_day_20_part_1(&input("input/day-20.txt"))),
        (20, 2) => println!("{}", run_day_20_part_2(&input("input/day-20.txt"))),
        _ => (),
    };
}

/// Wraps around [std::fs::read_to_string].
fn input<P>(path: P) -> String
where
    P: AsRef<std::path::Path>,
{
    std::fs::read_to_string(path).unwrap()
}
