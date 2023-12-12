use crate::common::div;

pub fn search_lhs(t: i64, d: i64) -> i64 {
    let from = 0;
    let to = peak(t).ceil() as i64 - 1;
    let insertion_point = binary_search(t, d, from, to);
    find_precise(t, d, insertion_point)
}

pub fn search_rhs(t: i64, d: i64) -> i64 {
    let from = peak(t).floor() as i64 - 1;
    let to = t;
    let insertion_point = binary_search_inv(t, d, from, to);
    find_precise(t, d, insertion_point)
}

pub(crate) fn fx(p: i64, t: i64) -> i64 {
    (t - p) * p
}

fn peak(t: i64) -> f64 {
    (t as f64) / 2.0
}

fn find_precise(t: i64, d: i64, estimate: i64) -> i64 {
    // check around the binary search result, just in case :)
    let needle;
    if estimate < 0 {
        needle = -1 * (estimate + 1);
    } else {
        needle = estimate;
    }

    return if d < fx(needle - 1, t) {
        needle - 1
    } else if d < fx(needle, t) {
        needle
    } else {
        needle + 1
    };
}

fn binary_search_inv(t: i64, d: i64, from: i64, to: i64) -> i64 {
    let mut low = from;
    let mut high = to;

    while low <= high {
        let m = div(low + high, 2);
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

fn binary_search(t: i64, d: i64, from: i64, to: i64) -> i64 {
    let mut low = from;
    let mut high = to;

    while low <= high {
        let m = div(low + high, 2);
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
