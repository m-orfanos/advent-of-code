use std::io::{self, BufRead};

fn main() {
    let mut ans = 0;
    for input in io::stdin().lock().lines() {
        let line = input.unwrap();
        let steps: Vec<&str> = line.split(",").collect();
        for step in steps {
            ans += hash(step);
        }
    }
    println!("{}", ans);
}

fn hash(s: &str) -> u32 {
    let mut ans = 0;
    for ch in s.chars() {
        ans += ch as u32;
        ans *= 17;
        ans %= 256;
    }
    ans
}
