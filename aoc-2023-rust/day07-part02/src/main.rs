use std::{
    cmp,
    collections::HashMap,
    io::{self, BufRead},
};

use crates::{
    common::day07::{compute_hands, Hand, HandType, Player, CARD_RANKS, PRIMES},
    parsers::split,
};

pub static CARD_RANKS2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

fn main() {
    let hands = compute_hands();
    let mut prime_hash_to_hand: HashMap<u64, Hand> = HashMap::new();
    for h in hands {
        prime_hash_to_hand.insert(h.prime_hash, h);
    }

    let jack_hash = build_jack_hash();
    let mut players = Vec::new();
    for line_res in io::stdin().lock().lines() {
        let line1 = line_res.unwrap();
        let line2 = split(&line1, " ");

        let cards_str1 = &line2[0];
        let bid = u64::from_str_radix(&line2[1], 10).unwrap();

        // parse input, compute hash/hand_type
        let (cards, prime_hash1) = compute_cards_and_prime_hash(cards_str1);
        let hand1 = prime_hash_to_hand[&prime_hash1];
        let player1 = Player {
            cards_str: cards_str1.to_string(),
            bid,
            cards,
            hand_type: hand1.hand_type,
            bit_hash: hand1.bit_hash,
        };

        let has_wild_card = hand1.bit_hash & (jack_hash) > 0;
        if !has_wild_card {
            players.push(player1);
            continue;
        }

        let card_ch = evaluate_wild_card_hand(&player1);
        let cards_str2 = cards_str1.replace("J", &card_ch.to_string());
        let (_, prime_hash2) = compute_cards_and_prime_hash(&cards_str2);
        let hand2 = prime_hash_to_hand[&prime_hash2];
        let player2 = Player {
            cards_str: cards_str1.to_string(), // display the original cards, might be useful for loggin
            bid,
            cards,
            bit_hash: hand2.bit_hash,
            hand_type: hand2.hand_type,
        };
        players.push(player2);
    }

    sort_hands(&mut players);

    let mut winnings = 0;
    for (i, p) in players.iter().enumerate() {
        winnings += p.bid * (1 + i as u64);
    }
    println!("{:?}", winnings);
}

fn compute_cards_and_prime_hash(cards_str: &String) -> ([usize; 5], u64) {
    let mut cards = [0, 0, 0, 0, 0];
    let mut hash = 1;
    for (i, card) in cards_str.chars().enumerate() {
        for (j, rank) in CARD_RANKS.iter().enumerate() {
            if card == *rank {
                hash *= PRIMES[j];
                break;
            }
        }
        for (j, rank) in CARD_RANKS2.iter().enumerate() {
            if card == *rank {
                cards[i] = j;
                break;
            }
        }
    }
    (cards, hash)
}

fn evaluate_wild_card_hand(player: &Player) -> char {
    // jacks act as a wildcard
    // 5k
    // JJJJJ -> AAAAA -> swap penta
    //
    // 4k
    // AAAAJ -> AAAAA -> swap quads
    // AJJJJ -> AAAAA -> swap high
    //
    // fh
    // AAAJJ -> AAAAA -> swap trips
    // AAJJJ -> AAAAA -> swap pair
    //
    // 3k
    // AAAKJ -> AAAKA -> swap trips
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
    let card_ch;

    // compute highest
    match player.hand_type {
        HandType::HighCard => {
            card_ch = find_high_rank(player);
        }
        HandType::OnePair => {
            let is_jack_pair = (player.bit_hash >> 13) & (1 << 9) > 0;
            if is_jack_pair {
                card_ch = find_high_rank(player);
            } else {
                card_ch = find_pair_rank(player);
            }
        }
        HandType::TwoPair => {
            card_ch = find_pair_rank(player);
        }
        HandType::ThreeKind => {
            let is_jack_trip = (player.bit_hash >> (1 * 26)) & (1 << 9) > 0;
            if is_jack_trip {
                card_ch = find_high_rank(player);
            } else {
                card_ch = find_trips_rank(player);
            }
        }
        HandType::FullHouse => {
            let is_jack_trip = (player.bit_hash >> (2 * 13)) & (1 << 9) > 0;
            if is_jack_trip {
                card_ch = find_pair_rank(player);
            } else {
                card_ch = find_trips_rank(player);
            }
        }
        HandType::FourKind => {
            let is_jack_quads = (player.bit_hash >> (3 * 13)) & (1 << 9) > 0;
            if is_jack_quads {
                card_ch = find_high_rank(player);
            } else {
                card_ch = find_quads_rank(player);
            }
        }
        HandType::FiveKind => {
            card_ch = 'A';
        }
    }
    card_ch
}

fn find_high_rank(player: &Player) -> char {
    let card_rank = player.bit_hash & 0b1110111111111;
    CARD_RANKS[(64 - card_rank.leading_zeros() - 1) as usize]
}

fn find_pair_rank(player: &Player) -> char {
    let card_rank = (player.bit_hash >> (1 * 13)) & 0b1110111111111;
    CARD_RANKS[(64 - card_rank.leading_zeros() - 1) as usize]
}

fn find_trips_rank(player: &Player) -> char {
    let card_rank = (player.bit_hash >> (2 * 13)) & 0b1110111111111;
    CARD_RANKS[(64 - card_rank.leading_zeros() - 1) as usize]
}

fn find_quads_rank(player: &Player) -> char {
    let card_rank = (player.bit_hash >> (3 * 13)) & 0b1110111111111;
    CARD_RANKS[(64 - card_rank.leading_zeros() - 1) as usize]
}

fn build_jack_hash() -> u64 {
    let xs = [0, 1, 2, 3];
    let mut a = 0;
    for x in xs {
        a = a | (1 << 9 + (13 * x));
    }
    a
}

fn sort_hands(players: &mut Vec<Player>) {
    players.sort_by(|a, b| {
        let cmp = a.hand_type.cmp(&b.hand_type);
        if cmp.is_eq() {
            // this isn't very fast, but it shouldn't
            // happen that often (surely)
            for (i, c1) in a.cards.iter().enumerate() {
                let c2 = b.cards[i];
                if c1.cmp(&c2).is_ne() {
                    return c1.cmp(&c2);
                }
            }
            return cmp::Ordering::Equal;
        }
        cmp
    });
}
