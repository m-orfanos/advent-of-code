// use std::{fs::File, io::BufReader};

use clap::{Parser};

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
    let cli =  Cli::parse();

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
