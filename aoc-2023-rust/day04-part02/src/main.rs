use crates::parsers::{parse_i64s, split};
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let mut ans = 0;
    let mut copies = HashMap::from([(0, 0)]);
    let mut line_nb = 0;

    for line_res in io::stdin().lock().lines() {
        let line = line_res.unwrap();

        // at least one ticket, i.e. the original
        if !copies.contains_key(&line_nb) {
            copies.insert(line_nb, 0);
        }
        copies.insert(line_nb, copies[&line_nb] + 1);

        // being parse input
        let card = split(&line, ":");
        let nbs = split(&card[1], "|");

        let mut winning_nbs = parse_i64s(&nbs[0], " ");
        winning_nbs.sort();

        let mut player_nbs = parse_i64s(&nbs[1], " ");
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
