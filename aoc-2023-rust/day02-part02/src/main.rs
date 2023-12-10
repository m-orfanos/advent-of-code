use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let mut sum_powers = 0;
    for input_res in io::stdin().lock().lines() {
        let mut colours_map = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        match input_res {
            Ok(input) => {
                let game_rounds: Vec<&str> = input.trim().split(":").map(|x| x.trim()).collect();
                for round_str in game_rounds[1].split(";").map(|x| x.trim()) {
                    for set_str in round_str.split(",").map(|x| x.trim()) {
                        let set = set_str.split(" ").map(|x| x.trim()).collect::<Vec<&str>>();
                        let n = i32::from_str_radix(set[0], 10).unwrap();
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
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }

    println!("{}", sum_powers);
}
