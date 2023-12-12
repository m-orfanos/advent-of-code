use std::io::{self, BufRead};
use crates::common::day06::{search_lhs, search_rhs};
use crates::parsers::parse_heading;

fn parse(s: &str) -> i64 {
    let mut ans = 0;
    for ch in s.chars() {
        if ch.is_digit(10) {
            ans = ans * 10 + ch.to_digit(10).unwrap() as i64;
        }
    }
    ans
}

fn main() {
    // parse input
    let mut t: i64 = 0;
    let mut d: i64 = 0;
    for line_res in io::stdin().lock().lines() {
        let line = line_res.unwrap();
        if line.starts_with("Time") {
            let ts = parse_heading(&line)[1];
            t = parse(ts);
        } else if line.starts_with("Distance") {
            let ds = parse_heading(&line)[1];
            d = parse(ds);
        }
    }

    // imagine graphing a function (the maths kind)
    //
    // for every 1ms pressing the button, p,
    // - the time travelled is (t-p) and
    // - the distance covered is velocity x time
    // - the velocity is p
    //
    // f(p, t) = (t-p)p = -p^2 + pt
    //
    // t=7
    // p  d w
    // 0  0
    // 1  6
    // 2 10 Y
    // 3 12 Y
    // 4 12 Y
    // 5 10 Y
    // 6  6
    // 7  0
    // margin = 4
    //
    // where does the max occur?
    // take derivative wrt p (t is constant) and set to 0
    // 0 = -2p + t -> 2p = t -> p = t/2
    //
    // we can now use binary search to quickly find
    // the interval containing all winning moves

    let lhs = search_lhs(t, d);
    let rhs = search_rhs(t, d);
    println!("{:?}", rhs - lhs + 1);
}
