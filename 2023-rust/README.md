# Advent of Code 2023

This project contains solutions to [Advent of Code](https://adventofcode.com/about) (AoC) puzzles in 
[Rust](https://www.rust-lang.org/).

## Installation

Prerequisites:

  - terminal with bash/zsh/etc
  - [Rust](https://www.rust-lang.org/tools/install)

Clone the repository and build the CLI app.

```shell
$ git clone git@github.com:m-orfanos/advent-of-code.git
$ cd advent-of-code/2023-rust
$ cargo build --release
```

## Usage

The CLI can be used to solve any day's puzzle with any valid input.

example using cargo 1

```shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/aoc`
error: the following required arguments were not provided:
  <DAY>
  <PART>
  <FILE>

Usage: aoc <DAY> <PART> <FILE>
```

example using cargo 2

```shell
$ cd /path/to/repo
$ cargo run -- 1 1 ./src/resources/aoc-2023-day01-part01

    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/aoc 1 1 ./src/resources/aoc-2023-day01-part01`
Day: 1
Part: 1
Input file: "./src/resources/aoc-2023-day01-part01"
File content:
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
```

example using release 1

```shell
$ cd /path/to/repo
$ ./target/release/aoc
Usage: aoc <DAY> <PART> <FILE>
````

example using release 2

```shell
# create a symbolic link
$ sudo ln -s /path/to/repo/target/release/aoc ~/.local/bin/aoc
$ aoc 1 1 /path/to/repo/src/resources/aoc-2023-day01-part01
Day: 1
Part: 1
Input file: "./src/resources/aoc-2023-day01-part01"
File content:
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
```
