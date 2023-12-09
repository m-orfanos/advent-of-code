use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let digit_words: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in stdin.lock().lines() {
        for word in digit_words {
            println!("{}", word);
        }
    }
}
