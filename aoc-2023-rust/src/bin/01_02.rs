use std::io::{self, BufRead};

fn main() {
    let words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut ans = 0;

    for line in io::stdin().lock().lines() {
        let haystack = line.unwrap();

        let mut calibrations = vec![];

        // parse digits
        for (i, ch) in haystack.chars().enumerate() {
            if ch.is_digit(10) {
                calibrations.push((i, ch.to_digit(10).unwrap()));
            }
        }

        // find word matches
        for (nb, w) in words.iter().enumerate() {
            match haystack.find(w) {
                Some(i) => {
                    calibrations.push((i, nb as u32));
                }
                None => (),
            }
            match haystack.rfind(w) {
                Some(i) => {
                    calibrations.push((i, nb as u32));
                }
                None => (),
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
