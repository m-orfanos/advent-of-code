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

    // let filepath = "resources/day01-ex.txt";
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
            ("01_01", "01_01_01", "01-ex.txt"),
            ("01_01", "01_01_02", "01.txt"),
            ("01_02", "01_02_01", "01-ex.txt"),
            ("01_02", "01_02_02", "01.txt"),
            // day 2
            ("02_01", "02_01_01", "02-ex.txt"),
            ("02_01", "02_01_02", "02.txt"),
            ("02_02", "02_02_01", "02-ex.txt"),
            ("02_02", "02_02_02", "02.txt"),
            // day 3
            ("03_01", "03_01_01", "03-ex.txt"),
            ("03_01", "03_01_02", "03.txt"),
            ("03_02", "03_02_01", "03-ex.txt"),
            ("03_02", "03_02_02", "03.txt"),
            // day 4
            ("04_01", "04_01_01", "04-ex.txt"),
            ("04_01", "04_01_02", "04.txt"),
            ("04_02", "04_02_01", "04-ex.txt"),
            ("04_02", "04_02_02", "04.txt"),
            // day 5
            ("05_01", "05_01_01", "05-ex.txt"),
            ("05_01", "05_01_02", "05.txt"),
            ("05_02", "05_02_01", "05-ex.txt"),
            ("05_02", "05_02_02", "05.txt"),
            // day 6
            ("06_01", "06_01_01", "06-ex.txt"),
            ("06_01", "06_01_02", "06.txt"),
            ("06_02", "06_02_01", "06-ex.txt"),
            ("06_02", "06_02_02", "06.txt"),
            // day 7
            ("07_01", "07_01_01", "07-ex.txt"),
            ("07_01", "07_01_02", "07.txt"),
            ("07_02", "07_02_01", "07-ex.txt"),
            ("07_02", "07_02_02", "07.txt"),
            // day 8
            ("08_01", "08_01_01", "08-ex.txt"),
            ("08_01", "08_01_02", "08.txt"),
            ("08_02", "08_02_01", "08-ex2.txt"),
            ("08_02", "08_02_02", "08.txt"),
            // day 9
            ("09_01", "09_01_01", "09-ex.txt"),
            ("09_01", "09_01_02", "09.txt"),
            ("09_02", "09_02_01", "09-ex.txt"),
            ("09_02", "09_02_02", "09.txt"),
            // day 10
            ("10_01", "10_01_01", "10-ex.txt"),
            ("10_01", "10_01_02", "10.txt"),
            // day 11
            ("11_01", "11_01_01", "11-ex.txt"),
            ("11_01", "11_01_02", "11.txt"),
            // day 12
            ("12_01", "12_01_01", "12-ex.txt"),
            ("12_01", "12_01_02", "12.txt"),
            // day 13
            ("13_01", "13_01_01", "13-ex.txt"),
            ("13_01", "13_01_02", "13.txt"),
            // day 14
            ("14_01", "14_01_01", "14-ex.txt"),
            ("14_01", "14_01_02", "14.txt"),
            // day 15
            ("15_01", "15_01_01", "15-ex.txt"),
            ("15_01", "15_01_02", "15.txt"),
            // day 16
            ("16_01", "16_01_01", "16-ex.txt"),
            ("16_01", "16_01_02", "16.txt"),
            // day 17
            ("17_01", "17_01_01", "17-ex.txt"),
            ("17_01", "17_01_02", "17.txt"),
            // day 18
            ("18_01", "18_01_01", "18-ex.txt"),
            ("18_01", "18_01_02", "18.txt"),
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
