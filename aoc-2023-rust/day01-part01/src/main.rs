use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut s: u32 = 0;
    let mut e: u32 = 0;
    let mut n: u32 = 0;

    for line in stdin.lock().lines() {
        for ch in line.as_ref().unwrap().chars() {
            if ch.is_digit(10) {
                s = ch.to_digit(10).unwrap();
                break;
            }
        }

        for ch in line.unwrap().chars().rev() {
            if ch.is_digit(10) {
                e = ch.to_digit(10).unwrap();
                break;
            }
        }

        n = n + s * 10 + e;
    }

    println!("{}", n);
}
