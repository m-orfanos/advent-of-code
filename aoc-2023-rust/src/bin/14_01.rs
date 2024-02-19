use std::io::{self, BufRead};

fn main() {
    // parse input
    let mut xxs = vec![];
    for input in io::stdin().lock().lines() {
        let line = input.unwrap();
        let xs: Vec<String> = line.chars().map(|ch| ch.to_string()).collect();
        xxs.push(xs);
    }

    slide_north(&mut xxs);

    let mut ans = 0;
    for i in 0..xxs.len() {
        for j in 0..xxs[i].len() {
            if xxs[i][j] == "O" {
                ans += xxs.len() - i;
            }
        }
    }
    println!("{}", ans);
}

fn slide_north(xxs: &mut Vec<Vec<String>>) {
    slide_north_0(xxs, (0..xxs[0].len()).into_iter().collect());
}

fn slide_north_0(xxs: &mut Vec<Vec<String>>, cols: Vec<usize>) {
    let mut ncols = vec![];
    for i in 0..xxs.len() - 1 {
        for j in &cols {
            let a = xxs[i][*j].to_string();
            let b = xxs[i + 1][*j].to_string();
            if a == "." && b == "O" {
                xxs[i][*j] = b;
                xxs[i + 1][*j] = a;

                // this column was modified
                // needs to be scanned again
                ncols.push(*j);
            }
        }
    }
    if !ncols.is_empty() {
        slide_north_0(xxs, ncols);
    }
}
