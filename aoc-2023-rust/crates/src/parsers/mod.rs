pub fn parse_heading(s: &String) -> Vec<&str> {
    s.split(":").map(|x| x.trim()).collect::<Vec<&str>>()
}

pub fn div(a: i64, b: i64) -> i64 {
    (a as f64 / b as f64) as i64
}
