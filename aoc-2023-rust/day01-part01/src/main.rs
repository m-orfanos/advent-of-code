use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut ans: u32 = 0;

    for line_res in stdin.lock().lines() {
        let calibration_value = match line_res {
            Ok(line) => {
                let s = find_digit(&line);
                let e = rfind_digit(&line);

                // build the number
                // s/e will always be single digit numbers
                s * 10 + e
            }
            // should never happen
            Err(e) => panic!("{:?}", e),
        };
        ans += calibration_value;
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
