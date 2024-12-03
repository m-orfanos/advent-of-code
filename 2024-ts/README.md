# README

## Prerequisites

- Terminal and/or VS Code
- Deno +2.x

## Getting Started

Create a `.env` file and add your session key

```text
# .env
AOC_SESSION=
```

Run via command line using the helper script

```shell
cd /path/to/project
./aoc.sh -d 01 -p 01
```

The script will download the puzzle input from the AoC
site and store it in the `src/resources` directory.

View help

```shell
Usage: aoc [OPTIONS...]

Optional flags:
  -h, --help       Display this help and exit
  -d, --day        The puzzle day (01-25) to solve
  -p, --part       The puzzle part (01, 02) to solve
```

Debug via VS Code runner

- Open "Run and Debug" (Ctrl + Shift + D)
- Click on the gear icon ("Open launch.json")
- Edit the deno `runtimeExecutable`
- Edit the "-d" and "-p" arguments as needed
- Add breakpoints as needed
- Run program (green button or F5)
