# advent-of-code-2023

Practise solving [Advent of Code 2023](https://adventofcode.com/2023) using Rust.

Quick hack -- input is never checked, unwrap() everywhere -- code will panic from any error.

To run the code against the examples, use `cargo test run_example`

To run the code against the heavier input data, use `cargo run <day> <part>`

> [!warning]
> Day 5 Part 2 is slow! Computing the result takes a bit of time. With a release build it takes close to 1 minute to complete on a M2 Max notebook. Same duration on a i7-12700 PC. This is with Rayon parallelism. Without Rayon, it takes around 8 and a half minutes.
