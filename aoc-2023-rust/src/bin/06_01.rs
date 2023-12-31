use aoc::common::day06::{search_lhs, search_rhs};
use aoc::parsers::{parse_i64s, split};
use std::io::{self, BufRead};

fn main() {
    // parse input
    let mut ts = Vec::new();
    let mut ds = Vec::new();
    for line_res in io::stdin().lock().lines() {
        let line = line_res.unwrap();
        if line.starts_with("Time") {
            ts = parse_i64s(&split(&line, ":")[1], " ");
        } else if line.starts_with("Distance") {
            ds = parse_i64s(&split(&line, ":")[1], " ");
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
    let mut res = Vec::new();
    for (t, d) in ts.into_iter().zip(ds.into_iter()) {
        // the search space is divided into two segments, LHS/RHS
        // LHS is always ascending, can use binary search as normal
        // RHS is always descending, need to tweak binary search a bit
        let lhs = search_lhs(t, d);
        let rhs = search_rhs(t, d);
        res.push((t, d, lhs, rhs, rhs - lhs + 1));
    }

    let mut ans = 1;
    for r in res {
        ans *= r.4;
    }
    println!("{:?}", ans);
}
