// use std::{fs::File, io::BufReader};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// AoC day
    #[arg()]
    day: i32,

    /// AoC day part
    #[arg()]
    part: i32,
}

fn main() {
    let cli = Cli::parse();

    // let filepath = "resources/day01-input-test";
    // let file = File::open(filepath);
    // let reader = BufReader::new(file.unwrap());

    match cli.day {
        1 => match cli.part {
            1 => todo!(),
            2 => todo!(),
            _ => todo!(),
        },
        _ => todo!(),
    }
}

#[cfg(test)]
mod aoc {
    use assert_cmd::Command;
    use std::fs::{read_to_string, File};
    use std::io::{BufRead, BufReader};

    #[test]
    fn tests() {
        let solutions_reader = BufReader::new(File::open("resources/solutions").unwrap());
        let solutions_str: Vec<_> = solutions_reader.lines().map(|l| l.unwrap()).collect();

        let tests = [
            ("01_01", "01_01_01", "day01-input-test"),
            ("01_01", "01_01_02", "day01-input"),
            ("01_02", "01_02_01", "day01-input-test"),
            ("01_02", "01_02_02", "day01-input"),
            // day 2
            ("02_01", "02_01_01", "day02-input-test"),
            ("02_01", "02_01_02", "day02-input"),
            ("02_02", "02_02_01", "day02-input-test"),
            ("02_02", "02_02_02", "day02-input"),
            // day 3
            ("03_01", "03_01_01", "day03-input-test"),
            ("03_01", "03_01_02", "day03-input"),
            ("03_02", "03_02_01", "day03-input-test"),
            ("03_02", "03_02_02", "day03-input"),
            // day 4
            ("04_01", "04_01_01", "day04-input-test"),
            ("04_01", "04_01_02", "day04-input"),
            ("04_02", "04_02_01", "day04-input-test"),
            ("04_02", "04_02_02", "day04-input"),
            // day 5
            ("05_01", "05_01_01", "day05-input-test"),
            ("05_01", "05_01_02", "day05-input"),
            ("05_02", "05_02_01", "day05-input-test"),
            ("05_02", "05_02_02", "day05-input"),
            // day 6
            ("06_01", "06_01_01", "day06-input-test"),
            ("06_01", "06_01_02", "day06-input"),
            ("06_02", "06_02_01", "day06-input-test"),
            ("06_02", "06_02_02", "day06-input"),
            // day 7
            ("07_01", "07_01_01", "day07-input-test"),
            ("07_01", "07_01_02", "day07-input"),
            ("07_02", "07_02_01", "day07-input-test"),
            ("07_02", "07_02_02", "day07-input"),
            // day 8
            ("08_01", "08_01_01", "day08-input-test"),
            ("08_01", "08_01_02", "day08-input"),
            ("08_02", "08_02_01", "day08-input-test2"),
            ("08_02", "08_02_02", "day08-input"),
            // day 9
            ("09_01", "09_01_01", "day09-input-test"),
            ("09_01", "09_01_02", "day09-input"),
            ("09_02", "09_02_01", "day09-input-test"),
            ("09_02", "09_02_02", "day09-input"),
            // day 10
            ("10_01", "10_01_01", "day10-input-test"),
            ("10_01", "10_01_02", "day10-input"),
            // day 11
            ("11_01", "11_01_01", "day11-input-test"),
            ("11_01", "11_01_02", "day11-input"),
            // day 12
            ("12_01", "12_01_01", "day12-input-test"),
            ("12_01", "12_01_02", "day12-input"),
            // day 13
            ("13_01", "13_01_01", "day13-input-test"),
            ("13_01", "13_01_02", "day13-input"),
            // day 14
            ("14_01", "14_01_01", "day14-input-test"),
            ("14_01", "14_01_02", "day14-input"),
        ];

        for (cmd, solutions_key, resource) in tests {
            let day_part: Vec<&str> = cmd.split("_").collect();
            println!("Puzzle:{}-{}, Input:{}", day_part[0], day_part[1], resource);
            let path = format!("resources/{resource}");
            let data = read_to_string(path).unwrap();
            let solution_str: Vec<_> = solutions_str
                .iter()
                .find(|b| b.starts_with(&solutions_key))
                .unwrap()
                .split("=")
                .collect();
            let solution = solution_str[1].to_string();
            let mut cmd = Command::cargo_bin(cmd).unwrap();
            let assert = cmd.write_stdin(data).assert();
            assert.success().stdout(format!("{solution}\n"));
        }
    }
}
