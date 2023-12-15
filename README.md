# AoC2023

## Advent of Code 2023

The Advent of Code is an annual coding event that consists of daily programming puzzles throughout December.
Each day, a new puzzle is released, and participants are encouraged to solve it using their programming skills.
Moreover, each day consists of a problem that has two parts.
The second part of the problem is unlocked after the first part is solved.

This year, I decided to step out of my comfort zone and tackle the puzzles using a programming language that I am not familiar with.
The language I chose to use for this year's Advent of Code challenge is [Rust](https://www.rust-lang.org/).
I have never used Rust before, but I have heard good things about it.
By doing so, I hope to expand my knowledge and improve my problem-solving abilities.

This documentation comment provides an overview of my approach for this year's Advent of Code challenge.

## Table of Contents

Each day of the Advent of Code challenge has its own folder.
The table below provides an overview of the folders and their contents.

| Folder | Description |
| ------ | ----------- |
| [Day 1](Day%201) | Day 1: [Chronal Calibration](https://adventofcode.com/2018/day/1) |

## Folder Structure

Each day of the Advent of Code challenge has its own folder.
The folder name is the day of the challenge, with the first letter capitalized.
For example, the folder for the first day of the challenge is named `Day 1`.

Each folder contains the following files:

| File | Description |
| ---- | ----------- |
| `README.md` | A description of the puzzle and my approach to solving it. |
| `sample.in` | A sample input file. |
| `input.in` | The input file. |
| `part1.rs` | The solution to the first part of the puzzle. |
| `part2.rs` | The solution to the second part of the puzzle. |

## Running the Code

To run the code, you need to have the [Rust compiler](https://www.rust-lang.org/tools/install) installed.
Once you have installed the Rust compiler, you can run the code using the following command:

```bash
rustc <filename>.rs < <inputfile>.in
```

For example, to run the code for the first part of the puzzle for Day 1, you can use the following command:

```bash
rustc part1.rs < input.in
```

## License

This project is licensed under the MIT License.
See the [LICENSE](LICENSE) file for details.
