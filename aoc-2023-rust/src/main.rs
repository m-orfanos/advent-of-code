mod d01_01;

use std::{collections::HashMap, path::PathBuf};

use clap::{arg, command, value_parser, ArgAction, Command};
use std::io::{self, BufRead};

fn main() {
    let mut puzzles: HashMap<i32, HashMap<i32, fn()>> = HashMap::new();
    puzzles.insert(1, HashMap::new());
    puzzles.get_mut(&1).and_then(|m| m.insert(1, d01_01::solve));

    let matches = command!() 
        .arg(arg!([day] "Day"))
        .arg(arg!([part] "Part"))
        .get_matches();

    let day = matches.get_one::<String>("day").unwrap().parse::<i32>().unwrap();
    let part = matches.get_one::<String>("part").unwrap().parse::<i32>().unwrap();

    puzzles.get(&day).and_then(|m| m.get(&part)).and_then(|f| Some(f()));
}
