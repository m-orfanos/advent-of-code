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
        let game_rounds: Vec<&str> = line.split(":").map(|x| x.trim()).collect();

        // parse Game, format
        // Game x
        let game: Vec<&str> = game_rounds[0].split(" ").map(|x| x.trim()).collect();
        let game_id = i32::from_str_radix(game[1], 10).unwrap();

        // parse rounds, format
        // n1 colour1, n2 colour2; m1 colour2, m2 colour3; ...
        let mut is_possible = true;
        for round_str in game_rounds[1].split(";").map(|x| x.trim()) {
            // parse round, format
            // n1 colour1, n2 colour2, ...
            for set_str in round_str.split(",").map(|x| x.trim()) {
                // parse set, format
                // n1 colour1
                let set: Vec<&str> = set_str.split(" ").map(|x| x.trim()).collect();
                let n = i32::from_str_radix(&set[0], 10).unwrap();
                if n > colours_map[&set[1]] {
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
