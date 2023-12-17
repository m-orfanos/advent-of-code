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
    use std::collections::HashMap;
    use std::fs::{read_to_string, File};
    use std::io::{BufRead, BufReader};

    #[test]
    fn tests() {
        let solutions_reader = BufReader::new(File::open("resources/solutions").unwrap());
        let solutions_str: Vec<_> = solutions_reader.lines().map(|l| l.unwrap()).collect();

        let days = ["01", "02", "03", "04", "05", "06", "07", "08", "09"];
        let parts = ["01", "02"];
        let resources_test = HashMap::from([("08_02_01", "2")]);
        for day in days {
            for part in parts {
                println!("Day {} Part {}...", day, part);

                let cmd = format!("{day}_{part}");
                let mut cmd = Command::cargo_bin(cmd).unwrap();

                // test input
                let k1 = format!("{day}_{part}_01");
                let with_override = resources_test.get(k1.as_str()).unwrap_or(&"");
                let path1 = format!("resources/day{day}-input-test{with_override}");
                let data1 = read_to_string(path1).unwrap();
                let solution_str1: Vec<_> = solutions_str
                    .iter()
                    .find(|b| b.starts_with(&k1))
                    .unwrap()
                    .split("=")
                    .collect();
                let p1 = solution_str1[1].to_string();

                let assert1 = cmd.write_stdin(data1).assert();
                assert1.success().stdout(format!("{p1}\n"));

                // puzzle input
                let k2 = format!("{day}_{part}_02");
                let path2 = format!("resources/day{day}-input");
                let data2 = read_to_string(path2).unwrap();
                let solution_str2: Vec<_> = solutions_str
                    .iter()
                    .find(|b| b.starts_with(&k2))
                    .unwrap()
                    .split("=")
                    .collect();
                let p2 = solution_str2[1].to_string();

                let assert2 = cmd.write_stdin(data2).assert();
                assert2.success().stdout(format!("{p2}\n"));
            }
        }
    }
}
