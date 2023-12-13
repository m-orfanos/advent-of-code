use std::{cmp, collections::HashMap};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
    FiveAces,
}

pub static CARD_RANKS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

pub static PRIMES: [u64; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

pub fn compute_hash_to_type(ranks: &[char; 13], primes: &[u64; 13]) -> HashMap<u64, HandType> {
    // this looks like a lot of work, but there are only 6188 combinations
    //
    // how to determine a player's rank?
    // the order is not important
    // there are no straights/flushes to consider
    // 5c = 1 unique rank
    // 4k = 2 unique ranks
    // fh = 2 unique ranks
    // 3k = 3 unique ranks
    // 2p = 3 unique ranks
    // 1p = 4 unique ranks
    // hc = 5 unique ranks
    //
    let mut hands = HashMap::new();

    // // compute 5K
    for (i, r) in ranks.iter().enumerate() {
        let hash = compute_hash(primes, i, i, i, i, i);
        if *r == 'A' {
            hands.insert(hash, HandType::FiveAces);
        } else {
            hands.insert(hash, HandType::FiveKind);
        }
    }

    // computer 4k and fh
    for (n1, n2, hand_type) in [
        (4, 1, HandType::FourKind),
        (1, 4, HandType::FourKind),
        (3, 2, HandType::FullHouse),
        (2, 3, HandType::FullHouse),
    ] {
        for i in 0..ranks.len() {
            for j in (i + 1)..ranks.len() {
                let h1 = u64::pow(primes[i], n1);
                let h2 = u64::pow(primes[j], n2);
                let hash = h1 * h2;
                hands.insert(hash, hand_type);
            }
        }
    }

    // computer 3k and 2p
    for (n1, n2, n3, hand_type) in [
        (3, 1, 1, HandType::ThreeKind),
        (1, 3, 1, HandType::ThreeKind),
        (1, 1, 3, HandType::ThreeKind),
        (2, 2, 1, HandType::TwoPair),
        (1, 2, 2, HandType::TwoPair),
        (2, 1, 2, HandType::TwoPair),
    ] {
        for i in 0..ranks.len() {
            for j in (i + 1)..ranks.len() {
                for k in (j + 1)..ranks.len() {
                    let h1 = u64::pow(primes[i], n1);
                    let h2 = u64::pow(primes[j], n2);
                    let h3 = u64::pow(primes[k], n3);
                    let hash = h1 * h2 * h3;
                    hands.insert(hash, hand_type);
                }
            }
        }
    }

    // // computer 1p
    for (n1, n2, n3, n4) in [(2, 1, 1, 1), (1, 2, 1, 1), (1, 1, 2, 1), (1, 1, 1, 2)] {
        for i in 0..ranks.len() {
            for j in (i + 1)..ranks.len() {
                for k in (j + 1)..ranks.len() {
                    for l in (k + 1)..ranks.len() {
                        let h1 = u64::pow(primes[i], n1);
                        let h2 = u64::pow(primes[j], n2);
                        let h3 = u64::pow(primes[k], n3);
                        let h4 = u64::pow(primes[l], n4);
                        let hash = h1 * h2 * h3 * h4;
                        hands.insert(hash, HandType::OnePair);
                    }
                }
            }
        }
    }

    // computer hc
    for i in 0..ranks.len() {
        for j in (i + 1)..ranks.len() {
            for k in (j + 1)..ranks.len() {
                for l in (k + 1)..ranks.len() {
                    for m in (l + 1)..ranks.len() {
                        let hash = compute_hash(primes, i, j, k, l, m);
                        hands.insert(hash, HandType::HighCard);
                    }
                }
            }
        }
    }

    hands
}

pub fn compute_hash(primes: &[u64; 13], i: usize, j: usize, k: usize, l: usize, m: usize) -> u64 {
    primes[i] * primes[j] * primes[k] * primes[l] * primes[m]
}

pub fn sort_hands(players: &mut Vec<(Vec<usize>, u64, &HandType)>) {
    players.sort_by(|a, b| {
        let cmp = a.2.cmp(&b.2);
        if cmp.is_eq() {
            // this isn't very fast, but it shouldn't
            // happen that often (surely)
            for (i, c1) in a.0.iter().enumerate() {
                let c2 = b.0[i];
                if c1.cmp(&c2).is_ne() {
                    return c1.cmp(&c2);
                }
            }
            return cmp::Ordering::Equal;
        }
        cmp
    });
}
