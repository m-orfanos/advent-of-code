/// calls str::split, trims result and collects into a Vec<String>
pub fn split(s: &String, pattern: &str) -> Vec<String> {
    s.split(pattern)
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>()
}

/// calls str::split, trims result, parses to i64 and collects into Vec<i64>
pub fn parse_ints(s: &String, delimiter: &str) -> Vec<i64> {
    s.split(delimiter)
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

pub fn parse_uints(s: &str, pattern: &str) -> Vec<u64> {
    s.split(pattern)
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

// create i64 from str
// equivalent to deleting all non-digit characters and
// parsing into an i64
pub fn parse_int(s: &str) -> i64 {
    let mut ans = 0;
    for ch in s.chars() {
        if ch.is_digit(10) {
            ans = ans * 10 + ch.to_digit(10).unwrap() as i64;
        }
    }
    ans
}
