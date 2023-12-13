use std::cmp;

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

pub struct Hand {
    pub bit_hash: u64,
    pub hand_type: HandType,
    pub prime_hash: u64,
}

pub fn compute_hands() -> Vec<Hand> {
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
    let mut hands = Vec::new();

    // // compute 5K
    for (i, r) in CARD_RANKS.iter().enumerate() {
        let ph = build_prime_hash(vec![(i, 1), (i, 1), (i, 1), (i, 1), (i, 1)]);

        let ranks_hash = 1 << i;

        if *r == 'A' {
            hands.push(build_hand(ph, ranks_hash, HandType::FiveAces));
        } else {
            hands.push(build_hand(ph, ranks_hash, HandType::FiveKind));
        }
    }

    // computer 4k and fh
    for (n1, n2, hand_type) in [
        (4, 1, HandType::FourKind),
        (1, 4, HandType::FourKind),
        (3, 2, HandType::FullHouse),
        (2, 3, HandType::FullHouse),
    ] {
        for i in 0..CARD_RANKS.len() {
            for j in (i + 1)..CARD_RANKS.len() {
                let ph = build_prime_hash(vec![(i, n1), (j, n2)]);

                let ii = 1 << (i * n1 as usize);
                let jj = 1 << (j * n2 as usize);
                let rank_hash = 1 << i | 1 << j | ii | jj;

                hands.push(build_hand(ph, rank_hash, hand_type));
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
        for i in 0..CARD_RANKS.len() {
            for j in (i + 1)..CARD_RANKS.len() {
                for k in (j + 1)..CARD_RANKS.len() {
                    let ph = build_prime_hash(vec![(i, n1), (j, n2), (k, n3)]);

                    let ii = 1 << (i * n1 as usize);
                    let jj = 1 << (j * n2 as usize);
                    let kk = 1 << (k * n3 as usize);
                    let rank_hash = 1 << i | 1 << j | 1 << k | ii | jj | kk;

                    hands.push(build_hand(ph, rank_hash, hand_type));
                }
            }
        }
    }

    // // computer 1p
    for (n1, n2, n3, n4) in [(2, 1, 1, 1), (1, 2, 1, 1), (1, 1, 2, 1), (1, 1, 1, 2)] {
        for i in 0..CARD_RANKS.len() {
            for j in (i + 1)..CARD_RANKS.len() {
                for k in (j + 1)..CARD_RANKS.len() {
                    for l in (k + 1)..CARD_RANKS.len() {
                        let ph = build_prime_hash(vec![(i, n1), (j, n2), (k, n3), (l, n4)]);

                        let ii = 1 << ((1 + i) * n1 as usize);
                        let jj = 1 << ((1 + j) * n2 as usize);
                        let kk = 1 << ((1 + k) * n3 as usize);
                        let ll = 1 << ((1 + l) * n4 as usize);
                        let rank_hash =
                            (1 << i) | (1 << j) | (1 << k) | (1 << l) | ii | jj | kk | ll;

                        hands.push(build_hand(ph, rank_hash, HandType::OnePair));
                    }
                }
            }
        }
    }

    // computer hc
    for i in 0..CARD_RANKS.len() {
        for j in (i + 1)..CARD_RANKS.len() {
            for k in (j + 1)..CARD_RANKS.len() {
                for l in (k + 1)..CARD_RANKS.len() {
                    for m in (l + 1)..CARD_RANKS.len() {
                        let ph = build_prime_hash(vec![(i, 1), (j, 1), (k, 1), (l, 1), (m, 1)]);
                        let ranks = 1 << i | 1 << j | 1 << k | 1 << l | 1 << m;
                        hands.push(build_hand(ph, ranks, HandType::HighCard));
                    }
                }
            }
        }
    }

    hands
}

fn build_prime_hash(vs: Vec<(usize, u32)>) -> u64 {
    vs.iter().fold(1, |acc, e| acc * u64::pow(PRIMES[e.0], e.1))
}

fn build_hand(prime_hash: u64, bit_hash: u64, hand_type: HandType) -> Hand {
    Hand {
        prime_hash,
        bit_hash,
        hand_type,
    }
}

pub fn compute_hash(primes: [u64; 13], i: usize, j: usize, k: usize, l: usize, m: usize) -> u64 {
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
