use std::io::{self, BufRead};

fn main() {
    let mut ans: u32 = 0;
    for line_res in io::stdin().lock().lines() {
        let line = line_res.unwrap();

        let s = find_digit(&line);
        let e = rfind_digit(&line);

        // build the number
        // s/e will always be single digit numbers
        ans += s * 10 + e
    }

    println!("{}", ans);
}

fn find_digit(line: &String) -> u32 {
    // using unwrap because there will ALWAYS be a value present
    line.chars()
        .find(|ch| ch.is_digit(10))
        .map(|ch| ch.to_digit(10))
        .unwrap()
        .unwrap()
}

fn rfind_digit(line: &String) -> u32 {
    // using unwrap because there will ALWAYS be a value present
    line.chars()
        .rev()
        .find(|ch| ch.is_digit(10))
        .map(|ch| ch.to_digit(10))
        .unwrap()
        .unwrap()
}
