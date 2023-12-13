use crates::{
    common::day07::{compute_hash_to_type, sort_hands, CARD_RANKS, PRIMES},
    parsers::split,
};
use std::io::{self, BufRead};

fn main() {
    let hash_to_hand = compute_hash_to_type(&CARD_RANKS, &PRIMES);

    let mut players = Vec::new();
    for line_res in io::stdin().lock().lines() {
        let line1 = line_res.unwrap();
        let line2 = split(&line1, " ");

        let cards_str = &line2[0];
        let bid = u64::from_str_radix(line2[1], 10).unwrap();

        // parse input, compute hash/hand_type
        let mut hand = Vec::new();
        let mut hash = 1;
        for card in cards_str.chars() {
            for (j, card_rank) in CARD_RANKS.iter().enumerate() {
                if card == *card_rank {
                    hash *= PRIMES[j];
                    hand.push(j);
                    break;
                }
            }
        }
        let hand_type = &hash_to_hand[&hash];
        players.push((hand, bid, hand_type));
    }

    sort_hands(&mut players);

    let mut winnings = 0;
    for (i, p) in players.iter().enumerate() {
        winnings += p.1 * (1 + i as u64);
    }
    println!("{:?}", winnings);
}
