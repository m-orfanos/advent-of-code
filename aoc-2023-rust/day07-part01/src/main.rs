use crates::common::day07::Hand;
use crates::common::day07::CARD_RANKS;
use crates::common::day07::PRIMES;
use crates::{
    common::day07::{compute_hands, sort_hands},
    parsers::split,
};
use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let hands = compute_hands();
    let mut hash_to_hand: HashMap<u64, Hand> = HashMap::new();
    for h in hands {
        hash_to_hand.insert(h.prime_hash, h);
    }

    let mut players = Vec::new();
    for line_res in io::stdin().lock().lines() {
        let line1 = line_res.unwrap();
        let line2 = split(&line1, " ");

        let cards_str = &line2[0];
        let bid = u64::from_str_radix(line2[1], 10).unwrap();

        // parse input, compute hash/hand_type
        let mut cards = Vec::new();
        let mut hash = 1;
        for card in cards_str.chars() {
            for (j, rank) in CARD_RANKS.iter().enumerate() {
                if card == *rank {
                    hash *= PRIMES[j];
                    cards.push(j);
                    break;
                }
            }
        }
        let Hand {
            bit_hash: _,
            hand_type,
            prime_hash: _,
        }: &Hand = &hash_to_hand[&hash];
        players.push((cards, bid, hand_type));
    }

    sort_hands(&mut players);

    let mut winnings = 0;
    for (i, p) in players.iter().enumerate() {
        winnings += p.1 * (1 + i as u64);
    }
    println!("{:?}", winnings);
}
