/// calls str::split, trims result and collects into a Vec<String>
pub fn split<'a>(s: &'a str, pattern: &'a str) -> Vec<&'a str> {
    s.split(pattern).map(|x| x.trim()).collect()
}

/// calls str::split, trims result, parses to i64 and collects into Vec<i64>
pub fn parse_i64s(s: &str, delimiter: &str) -> Vec<i64> {
    s.split(delimiter)
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn parse_u64s(s: &str, pattern: &str) -> Vec<u64> {
    s.split(pattern)
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect()
}

// create i64 from str
// equivalent to deleting all non-digit characters and
// parsing into an i64
pub fn parse_i64(s: &str) -> i64 {
    let mut ans = 0;
    for ch in s.chars() {
        if ch.is_digit(10) {
            ans = ans * 10 + ch.to_digit(10).unwrap() as i64;
        }
    }
    ans
}
