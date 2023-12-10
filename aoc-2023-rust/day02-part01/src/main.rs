use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let colours_map = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut sum_ids = 0;
    for input_res in io::stdin().lock().lines() {
        match input_res {
            Ok(input) => {
                let game_rounds: Vec<&str> = input.trim().split(":").map(|x| x.trim()).collect();

                let game: Vec<&str> = game_rounds[0].split(" ").map(|x| x.trim()).collect();
                let game_id = i32::from_str_radix(game[1], 10).unwrap();

                let mut is_possible = true;

                for round_str in game_rounds[1].split(";").map(|x| x.trim()) {
                    for set_str in round_str.split(",").map(|x| x.trim()) {
                        let set = set_str.split(" ").map(|x| x.trim()).collect::<Vec<&str>>();
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
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }

    println!("{}", sum_ids);
}
