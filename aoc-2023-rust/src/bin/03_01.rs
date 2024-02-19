use std::io::{self, BufRead};

fn main() {
    let mut parts: Vec<(u32, Vec<(i32, i32)>)> = vec![];
    let mut symbols: Vec<(i32, i32)> = vec![];

    for (i, input) in io::stdin().lock().lines().enumerate() {
        let line = input.unwrap();

        let mut coords = vec![];
        let mut n = 0;
        for (j, ch) in line.chars().enumerate() {
            if ch.is_digit(10) {
                n = n * 10 + ch.to_digit(10).unwrap();
                coords.push((i as i32, j as i32));
            } else {
                if ch != '.' {
                    symbols.push((i as i32, j as i32));
                }
                if n > 0 {
                    parts.push((n, coords));
                    // reset
                    coords = vec![];
                    n = 0;
                }
            }
        }

        if n > 0 {
            // edge of map
            parts.push((n, coords));
        }
    }

    let mut ans = 0;
    for part in parts {
        'next: for (px, py) in &part.1 {
            for (sx, sy) in &symbols {
                if &(px - 1) <= sx && sx <= &(px + 1) && &(py - 1) <= sy && sy <= &(py + 1) {
                    ans += part.0;
                    break 'next;
                }
            }
        }
    }

    println!("{}", ans);
}
