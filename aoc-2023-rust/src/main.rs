mod d01_01;
mod d01_02;

use clap::{arg, command, Parser};

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
    match cli.day {
        1 => match cli.part {
            1 => d01_01::solve(),
            2 => d01_02::solve(),
            _ => todo!(),
        },
        _ => todo!(),
    }
}
