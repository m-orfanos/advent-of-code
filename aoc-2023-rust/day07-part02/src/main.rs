use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use crates::{
    common::day07::{compute_hands, sort_hands, Hand, HandType, CARD_RANKS, PRIMES},
    parsers::split,
};

#[derive(Debug)]
struct Player {
    hand: Hand,
    cards_str: String,
    cards: Vec<usize>,
}

fn main() {
    let hands = compute_hands();
    let mut hash_to_hand: HashMap<u64, Hand> = HashMap::new();
    for h in hands {
        hash_to_hand.insert(h.prime_hash, h);
    }

    let mut players = Vec::new();
    let jack_hash = build_jack_hash();
    for line_res in io::stdin().lock().lines() {
        let line1 = line_res.unwrap();
        let line2 = split(&line1, " ");

        let cards_str = &line2[0];
        let bid = u64::from_str_radix(&line2[1], 10).unwrap();

        // parse input, compute hash/hand_type
        let player = compute_player_hand(&cards_str, &hash_to_hand);

        // evaluate again in case there's a jack
        evaluate_wild_card_hand(&player, &hash_to_hand, jack_hash);
        // println!("{:?}", p2.hand);

        // players.push((p1.hand, bid, player_hand.hand_type));
    }

    sort_hands(&mut players);

    let mut winnings = 0;
    for (i, p) in players.iter().enumerate() {
        winnings += p.1 * (1 + i as u64);
    }
    println!("{:?}", winnings);
}

fn evaluate_wild_card_hand(player: &Player, hash_to_hand: &HashMap<u64, Hand>, jack_hash: u64) {
    // jacks act as a wildcard
    // 5k
    // JJJJJ -> AAAAA -> swap
    //
    // 4k
    // AAAAJ -> AAAAA -> swap
    // AJJJJ -> AAAAA -> swap
    //
    // fh
    // AAAJJ -> AAAAA -> swap
    // AAJJJ -> AAAAA -> swap
    //
    // 3k
    // AAAKJ -> AAAKA -> swap 3k
    // KAJJJ -> KAAAA -> swap high
    //
    // 2p
    // KAAKJ -> KAAKA -> swap pair
    // AKKJJ -> AKKKK -> swap pair
    //
    // 1p
    // AKKQJ -> AAAKQ -> swap pair
    // AKQJJ -> AAAKQ -> swap high
    //
    // hc
    // AKQJT -> AAKQJ -> swap high
    let has_wild_card = player.hand.bit_hash & (jack_hash) > 0;
    if !has_wild_card {
        return;
    }

    // compute highest
    match player.hand.hand_type {
        HandType::HighCard => (),
        HandType::OnePair => (),
        HandType::TwoPair => (),
        HandType::ThreeKind => {
            // either JJJAB OR AAAJB
            let is_jack_trip = (player.hand.bit_hash >> 26) & (1 << 9) > 0;

            let card_rank;
            let card_ch;
            if is_jack_trip {
                // find highest card
                card_rank = player.hand.bit_hash & 0b1110111111111;
                card_ch = CARD_RANKS[(64 - card_rank.leading_zeros() - 1) as usize];
            } else {
                // find 3k
                card_rank = (player.hand.bit_hash >> 26) & 0b1110111111111;
                card_ch = CARD_RANKS[(64 - card_rank.leading_zeros() - 1) as usize];
            }
            let cards_str2 = player.cards_str.replace("J", &card_ch.to_string());
            let player = compute_player_hand(&cards_str2, &hash_to_hand);
            // return player;
        }
        HandType::FullHouse => (),
        HandType::FourKind => (),
        HandType::FiveKind => (),
        HandType::FiveAces => (),
    }
}

fn compute_player_hand(cards_str: &str, hash_to_hand: &HashMap<u64, Hand>) -> Player {
    let mut cards = Vec::new();
    let mut hash = 1;
    for card in cards_str.chars() {
        for (j, card_rank) in CARD_RANKS.iter().enumerate() {
            if card == *card_rank {
                hash *= PRIMES[j];
                cards.push(j);
                break;
            }
        }
    }
    Player {
        hand: hash_to_hand[&hash],
        cards,
        cards_str: cards_str.to_string(),
    }
}

fn build_jack_hash() -> u64 {
    let xs = [0, 1, 2, 3];
    let mut a = 0;
    for x in xs {
        a = a | (1 << 9 + (13 * x));
    }
    a
}
