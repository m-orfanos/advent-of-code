use regex::Regex;
use std::io::{self, BufRead};

fn main() {
    let mut ans = 0;

    // parse input
    for input in io::stdin().lock().lines() {
        let row: Vec<String> = input
            .unwrap()
            .trim()
            .split(" ")
            .map(|s| s.trim().to_string())
            .collect();

        let springs = &row[0];

        let groups: Vec<i32> = row[1]
            .to_string()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();

        // build a regex pattern to match against a possible arrangement
        // used to filter out invalid arrangements
        // pattern format: [^.]{a}[^#]+[^.]{b}[^#]+[^.]{c}
        let mut pattern = r"".to_string();
        for (i, g) in groups.iter().enumerate() {
            pattern.push_str(&format!("[^.]{{{g}}}"));
            if i < groups.len() - 1 {
                pattern.push_str("[^#]+");
            }
        }
        let re = Regex::new(&pattern).unwrap();

        // generate possible combinations
        let mut arrangements: Vec<String> = vec![];
        let mut processing = Vec::from([springs.to_string()]);
        while processing.len() > 0 {
            let curr = processing.pop().unwrap().to_string();
            let idx = curr.find('?');
            match idx {
                Some(_) => {
                    let next1 = curr.replacen("?", ".", 1);
                    if re.is_match(&next1) {
                        processing.push(next1);
                    }

                    let next2 = curr.replacen("?", "#", 1);
                    if re.is_match(&next2) {
                        processing.push(next2);
                    }
                }
                _ => {
                    let arrangement: Vec<&str> =
                        curr.split(".").filter(|xs| xs.len() > 0).collect();
                    // filter out remaining invalid combinations
                    if arrangement.len() == groups.len() {
                        let mut is_match = true;
                        for (i, g) in groups.iter().enumerate() {
                            if arrangement[i].len() != *g as usize {
                                is_match = false;
                                break;
                            }
                        }
                        if is_match {
                            arrangements.push(curr);
                        }
                    }
                }
            }
        }

        // println!("{} {:?} {}", springs, groups, arrangements.len());
        // for arragement in &arrangements {
        //     println!("{}", arragement);
        // }
        // println!("");

        ans += arrangements.len();
    }

    println!("{}", ans);
}
