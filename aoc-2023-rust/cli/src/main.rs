use clap::Parser;
use std::{fs::File, io::Read, path::PathBuf};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "A small CLI app used to run AoC puzzles"
)]
struct Args {
    #[arg(long_help = "The AoC puzzle day to run", required = true)]
    day: u32,

    #[arg(
        long_help = "The AoC puzzle day's part to run. Can be either 1 or 2.",
        required = true
    )]
    part: u32,

    #[arg(
        long_help = "The file to use as input for the day+part",
        required = true
    )]
    file: PathBuf,
}

fn main() {
    let args = Args::parse();
    println!("Day: {}", args.day);
    println!("Part: {}", args.part);
    println!("Input file: {:?}", args.file.to_string_lossy());

    let contents = read_file(args.file);
    println!("File content:\n{}", contents);
}

fn read_file(file: PathBuf) -> String {
    let path = file.as_path();
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    return match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => s,
    };
}
