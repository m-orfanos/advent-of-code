use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use crates::{
    common::day07::{compute_hands, sort_hands, Hand, HandType, CARD_RANKS, PRIMES},
    parsers::split,
};

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
        let mut hand = Vec::new();
        let mut hash = 1;

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
        for card in cards_str.chars() {
            for (j, card_rank) in CARD_RANKS.iter().enumerate() {
                if card == *card_rank {
                    hash *= PRIMES[j];
                    hand.push(j);
                    break;
                }
            }
        }

        let Hand {
            bit_hash: _,
            hand_type,
            prime_hash: _,
        }: &Hand = &hash_to_hand[&hash];
        // let has_wild_card = ranks & (1 << 9);
        // if has_wild_card != 0 {
        //     // compute highest
        //     match hand_type {
        //         HandType::HighCard => todo!(),
        //         HandType::OnePair => todo!(),
        //         HandType::TwoPair => todo!(),
        //         HandType::ThreeKind => todo!(),
        //         HandType::FullHouse => todo!(),
        //         HandType::FourKind => todo!(),
        //         HandType::FiveKind => todo!(),
        //         HandType::FiveAces => todo!(),
        //     }
        // }

        players.push((hand, bid, hand_type));
    }

    sort_hands(&mut players);

    let mut winnings = 0;
    for (i, p) in players.iter().enumerate() {
        winnings += p.1 * (1 + i as u64);
    }
    println!("{:?}", winnings);
}
