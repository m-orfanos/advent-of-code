use std::io::{self, BufRead};

fn main() {
    for line_res in io::stdin().lock().lines() {
        let line = line_res.unwrap();
    }
}
