use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();

    let mut ans = 0;
    let mut copies = HashMap::from([(0, 0)]);
    let mut line_nb = 0;

    for line_res in stdin.lock().lines() {
        let line = line_res.unwrap();

        // at least one ticket, i.e. the original
        if !copies.contains_key(&line_nb) {
            copies.insert(line_nb, 0);
        }
        copies.insert(line_nb, copies[&line_nb] + 1);

        // being parse input
        let card = line.split(":").map(|x| x.trim()).collect::<Vec<&str>>();
        let nbs = card[1].split("|").map(|x| x.trim()).collect::<Vec<&str>>();

        // parse winning numbers
        let mut winning_nbs = nbs[0]
            .split(" ")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        winning_nbs.sort();

        // parse player numbers
        let mut player_nbs = nbs[1]
            .split(" ")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        player_nbs.sort();

        // calculate nb matches
        let mut cnt = 0;
        for win_nb in winning_nbs {
            for pl_nb in &player_nbs {
                if pl_nb == &win_nb {
                    cnt += 1;
                    break;
                }
            }
        }

        // there are `nb_copies` of this ticket with `cnt` matches
        // the player has won an additional `nb_copies` of the next `cnt` tickets
        let nb_copies = copies[&line_nb];
        for i in 0..cnt {
            if !copies.contains_key(&(line_nb + 1 + i)) {
                copies.insert(line_nb + 1 + i, 0);
            }
            copies.insert(line_nb + 1 + i, copies[&(line_nb + 1 + i)] + nb_copies);
        }

        // increment line counter
        line_nb += 1;
    }

    // total number of tickets
    for e in copies {
        ans += e.1;
    }

    println!("{:?}", ans);
}
