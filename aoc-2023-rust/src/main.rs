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

// FIXME simplify below, iterate over a list of (day,part,input,answer) instead of wtv this is...
#[cfg(test)]
mod aoc {
    use assert_cmd::Command;
    use std::collections::HashMap;
    use std::fs::{read_to_string, File};
    use std::io::{BufRead, BufReader};

    #[test]
    fn tests() {
        let solutions_reader = BufReader::new(File::open("resources/solutions").unwrap());
        let solutions_str: Vec<_> = solutions_reader.lines().map(|l| l.unwrap()).collect();

        let days = [
            "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11",
        ];
        let parts = ["01", "02"];

        let resources: HashMap<&str, &str> = HashMap::from([
            // format: DAY_PART_FILENAME
            // day 1 input
            ("01_01_01", "day01-input-test"),
            ("01_01_02", "day01-input"),
            ("01_02_01", "day01-input-test"),
            ("01_02_02", "day01-input"),
            // day 2
            ("02_01_01", "day02-input-test"),
            ("02_01_02", "day02-input"),
            ("02_02_01", "day02-input-test"),
            ("02_02_02", "day02-input"),
            // day 3
            ("03_01_01", "day03-input-test"),
            ("03_01_02", "day03-input"),
            ("03_02_01", "day03-input-test"),
            ("03_02_02", "day03-input"),
            // day 4
            ("04_01_01", "day04-input-test"),
            ("04_01_02", "day04-input"),
            ("04_02_01", "day04-input-test"),
            ("04_02_02", "day04-input"),
            // day 5
            ("05_01_01", "day05-input-test"),
            ("05_01_02", "day05-input"),
            ("05_02_01", "day05-input-test"),
            ("05_02_02", "day05-input"),
            // day 6
            ("06_01_01", "day06-input-test"),
            ("06_01_02", "day06-input"),
            ("06_02_01", "day06-input-test"),
            ("06_02_02", "day06-input"),
            // day 7
            ("07_01_01", "day07-input-test"),
            ("07_01_02", "day07-input"),
            ("07_02_01", "day07-input-test"),
            ("07_02_02", "day07-input"),
            // day 8
            ("08_01_01", "day08-input-test"),
            ("08_01_02", "day08-input"),
            ("08_02_01", "day08-input-test2"), // only difference for now...
            ("08_02_02", "day08-input"),
            // day 9
            ("09_01_01", "day09-input-test"),
            ("09_01_02", "day09-input"),
            ("09_02_01", "day09-input-test"),
            ("09_02_02", "day09-input"),
            // day 10
            ("10_01_01", "day10-input-test"),
            ("10_01_02", "day10-input"),
            // day 11
            ("11_01_01", "day11-input-test"),
            ("11_01_02", "day11-input"),
        ]);

        for day in days {
            for part in parts {
                // test input
                let k1 = format!("{day}_{part}_01");
                let resource1 = resources.get(k1.as_str()).unwrap_or(&"").to_string();
                if resource1 == "" {
                    continue;
                }

                println!("Day {} Part {}...", day, part);

                let path1 = format!("resources/{resource1}");
                let data1 = read_to_string(path1).unwrap();
                let solution_str1: Vec<_> = solutions_str
                    .iter()
                    .find(|b| b.starts_with(&k1))
                    .unwrap()
                    .split("=")
                    .collect();
                let p1 = solution_str1[1].to_string();

                // launch command
                let cmd = format!("{day}_{part}");
                let mut cmd = Command::cargo_bin(cmd).unwrap();
                let assert1 = cmd.write_stdin(data1).assert();
                assert1.success().stdout(format!("{p1}\n"));

                // puzzle input
                let k2 = format!("{day}_{part}_02");
                let resource2 = resources.get(k2.as_str()).unwrap_or(&"").to_string();
                if resource2 == "" {
                    continue;
                }
                let path2 = format!("resources/{resource2}");
                let data2 = read_to_string(path2).unwrap();
                let solution_str2: Vec<_> = solutions_str
                    .iter()
                    .find(|b| b.starts_with(&k2))
                    .unwrap()
                    .split("=")
                    .collect();
                let p2 = solution_str2[1].to_string();

                // launch command
                let assert2 = cmd.write_stdin(data2).assert();
                assert2.success().stdout(format!("{p2}\n"));
            }
            println!("");
        }
    }
}
