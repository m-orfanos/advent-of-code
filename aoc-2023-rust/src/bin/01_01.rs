use std::io::{self, BufRead};

fn main() {
    let mut ans: u32 = 0;
    for line in io::stdin().lock().lines() {
        let haystack = line.unwrap();

        let mut calibrations = vec![];

        // parse digits
        for (i, ch) in haystack.chars().enumerate() {
            if ch.is_digit(10) {
                calibrations.push((i, ch.to_digit(10).unwrap()));
            }
        }

        calibrations.sort_by(|a, b| a.0.cmp(&b.0));

        // find first and last digits
        let n = calibrations.first().unwrap();
        let m = calibrations.last().unwrap();

        // build the number
        ans += n.1 * 10 + m.1;
    }

    println!("{}", ans);
}
