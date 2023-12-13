use crates::parsers::split;
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let colours_map = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut sum_ids = 0;
    for line_res in io::stdin().lock().lines() {
        let line = line_res.unwrap();

        // parse line, format
        // Game x: n1 colour1, n2 colour2; m1 colour2, m2 colour3; ...
        let game_rounds = split(&line, ":");

        // parse Game, format
        // Game x
        let game = split(game_rounds[0], " ");
        let game_id = i32::from_str_radix(game[1], 10).unwrap();

        // parse rounds, format
        // n1 colour1, n2 colour2; m1 colour2, m2 colour3; ...
        let mut is_possible = true;
        for round_str in split(game_rounds[1], ";") {
            // parse round, format
            // n1 colour1, n2 colour2, ...
            for set_str in split(round_str, ",") {
                // parse set, format
                // n1 colour1
                let set = split(set_str, " ");
                let n = i32::from_str_radix(set[0], 10).unwrap();
                let colour = set[1];
                if n > colours_map[colour] {
                    is_possible = false;
                    break;
                }
            }

            if !is_possible {
                break;
            }
        }

        if is_possible {
            sum_ids += game_id;
        }
    }

    println!("{}", sum_ids);
}
