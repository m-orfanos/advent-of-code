use std::io::{self, BufRead};
use aoc::parsers::{split, parse_i64s};

fn main() {
    let mut ans: i64 = 0;
    for line_res in io::stdin().lock().lines() {
        let line = line_res.unwrap();

        // start parse input
        let card = split(&line, ":");
        let nbs = split(&card[1], "|");

        let mut winning_nbs = parse_i64s(&nbs[0], " ");
        winning_nbs.sort();

        let mut player_nbs = parse_i64s(&nbs[1], " ");
        player_nbs.sort();

        // calculate points
        let mut cnt: i64 = 0;
        for win_nb in winning_nbs {
            for pl_nb in &player_nbs {
                if pl_nb > &win_nb {
                    break;
                }
                if pl_nb == &win_nb {
                    cnt += 1;
                }
            }
        }

        if cnt > 0 {
            ans += i64::pow(2, (cnt - 1) as u32);
        }
    }

    println!("{}", ans);
}
