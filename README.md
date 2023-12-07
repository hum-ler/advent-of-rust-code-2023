# advent-of-code-2023

Practise solving [Advent of Code 2023](https://adventofcode.com/2023) using Rust.

> [!warning]
> This is a quick hack -- input is never checked, unwrap() everywhere -- code will panic from any error.

To run the code against the examples, use `cargo test run_example`

To run the code against the heavier input data, use `cargo run <day> <part>`

> [!warning]
> Day 5 Part 2 is slow! Computing the result takes a bit of time. With a release build it takes close to 1 minute to complete on a M2 Max notebook. Same duration on a i7-12700K PC. This is with Rayon parallelism. Without Rayon, it takes around 8 and a half minutes.

## Answers

> [!note]
> Input differs for participants.

| Day | Part 1 | Part 2 |
|:-:|:-:|:-:|
| 1 | 55816 | 54980 |
| 2 | 2101 | 58269 |
| 3 | 546312 | 87449461 |
| 4 | 20107 | 8172507 |
| 5 | 836040384 | 10834440 |
| 6 | 1159152 | 41513103 |
| 7 | 241344943 | 243101568 |
