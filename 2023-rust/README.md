# Advent of Code 2023

This project contains solutions to [Advent of Code](https://adventofcode.com/about) (AoC) puzzles in 
[Rust](https://www.rust-lang.org/).

## Installation

Prerequisites:

- terminal with bash/zsh/etc
- [Rust](https://www.rust-lang.org/tools/install)

Clone the repository and build the CLI app.

```shell
git clone
cd advent-of-code/2023-rust
cargo build --release
```

## Usage

A bash utlity script is used to run the solutions, it takes the day and day-part as arguments.

```shell
# The `<DAY>` is TWO digits 01, 02, ..., 25
# The `<PART>` is TWO digits 01 or 02
cd /path/to/repo/2023-rust
./aoc <DAY> <PART>
```

> [!warning]
> AoC devs have requested puzzle inputs be excluded from git repos so I had to nuke them using `git rebase`. The scripts assume inputs are stored in a `resources` directory. See the scripts for naming scheme.

## Development

The repository contains vscode debug configurations for every puzzle. Each config passes the puzzle input to the script as an argument. Open the `Run and Debug` view/pane to see them.
