use std::collections::HashMap;

pub mod day05;
pub mod day06;
pub mod day07;

pub fn div(a: i64, b: i64) -> i64 {
    (a as f64 / b as f64) as i64
}

/// returns the prime factors for the given number
fn find_primes(n: i32) -> Vec<i32> {
    // this is horribly inefficient
    let mut t = n;
    let mut primes = vec![];
    while t % 2 == 0 {
        primes.push(2);
        t = t / 2;
    }

    let mut p = 3;
    while p * p <= t {
        while t % p == 0 {
            primes.push(p);
            t = t / p;
        }
        p += 2;
    }

    if t > 1 {
        primes.push(t);
    }

    primes
}

/// creates a map from the given vector
/// the keys are the numbers in the vector,
/// the values are the number of duplicates
fn compute_counter(ns: Vec<i32>) -> HashMap<i32, i32> {
    let mut map = HashMap::new();
    for n in ns {
        if map.contains_key(&n) {
            map.insert(n, map.get(&n).unwrap() + 1);
        } else {
            map.insert(n, 1);
        }
    }
    map
}

pub fn lcm(ns: Vec<i32>) -> u64 {
    // this is horribly inefficient
    let mut prime_to_count = HashMap::new();
    for n in ns {
        let primes = find_primes(n);
        let primes_to_count = compute_counter(primes);
        for (prime, curr) in primes_to_count {
            if prime_to_count.contains_key(&prime) {
                let next = curr.max(*prime_to_count.get(&prime).unwrap());
                prime_to_count.insert(prime, next + 0);
            } else {
                prime_to_count.insert(prime, curr + 0);
            }
        }
    }

    let mut lcms = vec![];
    for (prime, cnt) in prime_to_count {
        for _ in 0..cnt {
            lcms.push(prime);
        }
    }

    let mut ans: u64 = 1;
    for lcm in lcms {
        ans *= lcm as u64;
    }
    ans
}
