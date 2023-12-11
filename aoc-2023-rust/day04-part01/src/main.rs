use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut ans = 0;
    for line_res in stdin.lock().lines() {
        let line = line_res.unwrap();

        // start parse input
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

        // calculate points
        let mut cnt = 0;
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
            ans += u32::pow(2, cnt - 1);
        }
    }

    println!("{}", ans);
}
