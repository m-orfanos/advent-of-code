use std::io::{self, BufRead};

fn parse_heading(s: &String) -> Vec<&str> {
    s.split(":").map(|x| x.trim()).collect::<Vec<&str>>()
}

fn parse_arr(s: &str) -> Vec<i64> {
    s.split(" ")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn main() {
    // parse input
    let mut ts = Vec::new();
    let mut ds = Vec::new();
    for line_res in io::stdin().lock().lines() {
        let line = line_res.unwrap();
        if line.starts_with("Time") {
            ts = parse_arr(parse_heading(&line)[1]);
        } else if line.starts_with("Distance") {
            ds = parse_arr(parse_heading(&line)[1]);
        }
    }

    let mut res = Vec::new();

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
    for (t, d) in ts.into_iter().zip(ds.into_iter()) {
        // the search space is divided into two segments, LHS/RHS
        // LHS is always ascending, can use binary search as normal
        // RHS is always descending, need to tweat bserch a bit
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

fn search_rhs(t: i64, d: i64) -> i64 {
    let from = peak(t).floor() as i64 - 1;
    let to = t;
    let insertion_point = binary_search_inv(t, d, from, to);
    find_precise(t, d, insertion_point)
}

fn binary_search_inv(t: i64, d: i64, from: i64, to: i64) -> i64 {
    let mut low = from;
    let mut high = to;

    while low <= high {
        let m = idiv(low + high, 2);
        let fm = fx(m, t);
        if fm > d {
            low = m + 1;
        } else if fm < d {
            high = m - 1;
        } else {
            return m; // exact match
        }
    }

    -1 * (low + 1) // closest match
}

fn find_precise(t: i64, d: i64, estimate: i64) -> i64 {
    // check around the binary search result, just in case :)
    let needle;
    if estimate < 0 {
        needle = -1 * (estimate + 1);
    } else {
        needle = estimate;
    }

    if d < fx(needle - 1, t) {
        return needle - 1;
    } else if d < fx(needle, t) {
        return needle;
    } else {
        return needle + 1;
    }
}

fn search_lhs(t: i64, d: i64) -> i64 {
    let from = 0;
    let to = peak(t).ceil() as i64 - 1;
    let insertion_point = binary_search(t, d, from, to);
    find_precise(t, d, insertion_point)
}

fn binary_search(t: i64, d: i64, from: i64, to: i64) -> i64 {
    let mut low = from;
    let mut high = to;

    while low <= high {
        let m = idiv(low + high, 2);
        let fm = fx(m, t);
        if fm < d {
            low = m + 1;
        } else if fm > d {
            high = m - 1;
        } else {
            return m; // exact match
        }
    }

    -1 * (low + 1) // closest match
}

fn fx(p: i64, t: i64) -> i64 {
    (t - p) * p
}

fn peak(t: i64) -> f64 {
    (t as f64) / 2.0
}

fn idiv(a: i64, b: i64) -> i64 {
    (a as f64 / b as f64) as i64
}
