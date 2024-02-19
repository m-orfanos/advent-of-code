use aoc::common::day07::Hand;
use aoc::common::day07::Player;
use aoc::common::day07::CARD_RANKS;
use aoc::common::day07::PRIMES;
use aoc::{
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
        let bid = u64::from_str_radix(&line2[1], 10).unwrap();

        // parse input, compute hash/hand_type
        let mut cards = [0, 0, 0, 0, 0];
        let mut hash = 1;
        for (i, card) in cards_str.chars().enumerate() {
            for (j, rank) in CARD_RANKS.iter().enumerate() {
                if card == *rank {
                    hash *= PRIMES[j];
                    cards[i] = j;
                    break;
                }
            }
        }
        let hand = hash_to_hand[&hash];
        let p = Player {
            cards_str: cards_str.clone(),
            bid,
            cards,
            hand_type: hand.hand_type,
            bit_hash: hash,
        };
        players.push(p);
    }

    sort_hands(&mut players);

    let mut winnings = 0;
    for (i, p) in players.iter().enumerate() {
        winnings += p.bid * (1 + i as u64);
    }
    println!("{:?}", winnings);
}
