use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let mut sum_powers = 0;

    for input in io::stdin().lock().lines() {
        let line = input.unwrap();

        // reset map
        let mut colours_map = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        // parse line, format
        // Game x: n1 colour1, n2 colour2; m1 colour2, m2 colour3; ...
        let game_rounds: Vec<&str> = line.split(":").map(|x| x.trim()).collect();

        // parse rounds, format
        // n1 colour1, n2 colour2; m1 colour2, m2 colour3; ..
        for round_str in game_rounds[1].split(";").map(|x| x.trim()) {
            // parse round, format
            // n1 colour1, n2 colour2, ...
            for set_str in round_str.split(",").map(|x| x.trim()) {
                // parse set, format
                // n1 colour1
                let set: Vec<&str> = set_str.split(" ").map(|x| x.trim()).collect();
                let n: i32 = set[0].parse().unwrap();
                let colour = set[1];

                if n > colours_map[colour] {
                    colours_map.insert(colour, n);
                }
            }
        }

        let mut power = 1;
        for n in colours_map.values() {
            power *= n;
        }
        sum_powers += power;
    }

    println!("{}", sum_powers);
}
