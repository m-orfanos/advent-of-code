use crates::common::day06::{search_lhs, search_rhs};
use crates::parsers::{parse_int, split};
use std::io::{self, BufRead};

fn main() {
    // parse input
    let mut t: i64 = 0;
    let mut d: i64 = 0;
    for line_res in io::stdin().lock().lines() {
        let line = line_res.unwrap();
        if line.starts_with("Time") {
            t = parse_int(&split(&line, ":")[1]);
        } else if line.starts_with("Distance") {
            d = parse_int(&split(&line, ":")[1]);
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
