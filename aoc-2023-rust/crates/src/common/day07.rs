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

#[derive(Debug, Copy, Clone)]
pub struct Hand {
    /// the cards are encoded into a single 64-bit integer
    ///
    /// for example, the hand T55J5 is encoded as 536871680 = 0b100000000000000000001100000000
    /// this translates to:
    /// AKQJT98765432
    /// 0000000001000 3k <- 555
    /// 0000000000000 2p
    /// 0001100000000 hc <- JT
    ///
    /// more generally,
    /// <-----4x---->|<-----3x---->|<-----2x---->|<-----1x----> nb of copies
    /// AKQJT98765432|AKQJT98765432|AKQJT98765432|AKQJT98765432
    /// bbbbbbbbbbbbb|bbbbbbbbbbbbb|bbbbbbbbbbbbb|bbbbbbbbbbbbb
    pub bit_hash: u64,
    /// represents the "hand rank" for this hand
    /// e.g. high card, two pair, ...
    pub hand_type: HandType,
    /// the cards are encoded as a single 64-bit integer
    ///
    /// each card rank is mapped to a prime number
    /// all primes are multiplied together to compute a unique number
    ///
    /// for example, the hand T55J5 is encoded as
    /// 228781 = 23 * 7 * 7 * 29 * 7
    ///
    /// this allows quick lookups for arbitrary hands
    /// which is then used to retrieve the hand_type
    pub prime_hash: u64,
}

impl Hand {
    pub fn new(prime_hash: u64, bit_hash: u64, hand_type: HandType) -> Self {
        Hand {
            prime_hash,
            bit_hash,
            hand_type,
        }
    }
}

#[derive(Debug)]
pub struct Player {
    pub hand_type: HandType,
    pub cards: [usize; 5],
    pub bid: u64,
    pub bit_hash: u64,
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
    let mut hands = Vec::new();

    // compute 5K
    for (i, r) in CARD_RANKS.iter().enumerate() {
        let ph = build_prime_hash(vec![(i, 1), (i, 1), (i, 1), (i, 1), (i, 1)]);
        let bh = 1 << i;

        let ht = if *r == 'A' {
            HandType::FiveAces
        } else {
            HandType::FiveKind
        };

        hands.push(Hand::new(ph, bh, ht));
    }

    // compute 4k and fh
    for (n1, n2, hand_type) in [
        (4, 1, HandType::FourKind),
        (1, 4, HandType::FourKind),
        (3, 2, HandType::FullHouse),
        (2, 3, HandType::FullHouse),
    ] {
        for i in 0..CARD_RANKS.len() {
            for j in (i + 1)..CARD_RANKS.len() {
                let ph = build_prime_hash(vec![(i, n1), (j, n2)]);
                let bh = build_bit_hash(vec![(i, n1), (j, n2)]);

                hands.push(Hand::new(ph, bh, hand_type));
            }
        }
    }

    // compute 3k and 2p
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
                    let bh = build_bit_hash(vec![(i, n1), (j, n2), (k, n3)]);

                    hands.push(Hand::new(ph, bh, hand_type));
                }
            }
        }
    }

    // // compute 1p
    for (n1, n2, n3, n4) in [(2, 1, 1, 1), (1, 2, 1, 1), (1, 1, 2, 1), (1, 1, 1, 2)] {
        for i in 0..CARD_RANKS.len() {
            for j in (i + 1)..CARD_RANKS.len() {
                for k in (j + 1)..CARD_RANKS.len() {
                    for l in (k + 1)..CARD_RANKS.len() {
                        let ph = build_prime_hash(vec![(i, n1), (j, n2), (k, n3), (l, n4)]);
                        let bh = build_bit_hash(vec![(i, n1), (j, n2), (k, n3), (l, n4)]);

                        hands.push(Hand::new(ph, bh, HandType::OnePair));
                    }
                }
            }
        }
    }

    // compute hc
    for i in 0..CARD_RANKS.len() {
        for j in (i + 1)..CARD_RANKS.len() {
            for k in (j + 1)..CARD_RANKS.len() {
                for l in (k + 1)..CARD_RANKS.len() {
                    for m in (l + 1)..CARD_RANKS.len() {
                        let ph = build_prime_hash(vec![(i, 1), (j, 1), (k, 1), (l, 1), (m, 1)]);
                        let bh = build_bit_hash(vec![(i, 1), (j, 1), (k, 1), (l, 1), (m, 1)]);
                        hands.push(Hand::new(ph, bh, HandType::HighCard));
                    }
                }
            }
        }
    }

    hands
}

fn build_bit_hash(vs: Vec<(usize, u32)>) -> u64 {
    let mut bit_hash = 0;
    for (i, nb) in vs {
        bit_hash = bit_hash | (1 << (i + (13 * (nb - 1) as usize)));
    }
    bit_hash
}

fn build_prime_hash(vs: Vec<(usize, u32)>) -> u64 {
    vs.iter().fold(1, |acc, e| acc * u64::pow(PRIMES[e.0], e.1))
}

pub fn compute_hash(primes: [u64; 13], i: usize, j: usize, k: usize, l: usize, m: usize) -> u64 {
    primes[i] * primes[j] * primes[k] * primes[l] * primes[m]
}

pub fn sort_hands(players: &mut Vec<Player>) {
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
