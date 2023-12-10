# advent-of-rust-code-2023

Practise solving [Advent of Code 2023](https://adventofcode.com/2023) using Rust.

> [!warning]
> This is a quick hack -- input is never checked, unwrap() everywhere -- code will panic from any error.

To run the code against the examples, use `cargo test run_example`

To run the code against the heavier input data, use `cargo run <day> <part>`

> [!warning]
> Day 5 Part 2 is slow! Computing the result takes a bit of time. With a release build it takes close to 1 minute to complete on a M2 Max notebook. Same duration on a i7-12700K PC. This is with Rayon parallelism. Without Rayon, it takes around 8 and a half minutes.

> [!note]
> I couldn't solve Day 8 Part 2 without looking up the strategy online. The trick is to realize (by inspecting the sequence) that for every case of A node, the mapping always cycles around after the Z node. Once you have the length of the sequences, you can calculate the multiple that is when all the Z nodes coincide.

## Answers

> [!note]
> Input data differs between participants.

| Day | Part 1 | Part 2 |
|:-:|:-:|:-:|
| 1 | 55816 | 54980 |
| 2 | 2101 | 58269 |
| 3 | 546312 | 87449461 |
| 4 | 20107 | 8172507 |
| 5 | 836040384 | 10834440 |
| 6 | 1159152 | 41513103 |
| 7 | 241344943 | 243101568 |
| 8 | 16409 | 11795205644011 |
| 9 | 1696140818 | 1152 |
