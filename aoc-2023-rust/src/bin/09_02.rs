use std::io::{self, BufRead};

fn main() {
    let mut prediction = 0;

    for line_res in io::stdin().lock().lines() {
        let line = line_res.unwrap();

        // parse input
        let history: Vec<i64> = line.split(" ").map(|n| n.trim().parse().unwrap()).collect();

        // evaluate intermediary states
        let mut pyramid: Vec<Vec<i64>> = vec![];
        pyramid.push(history);
        loop {
            let next = get_next(&pyramid[pyramid.len() - 1]);
            let cnt = next.iter().filter(|n| n.eq(&&0)).count();
            if cnt == next.len() {
                break;
            }
            pyramid.push(next);
        }

        // make prediction
        let mut history_prediction = 0;
        for row in pyramid.iter().rev() {
            println!("{:?} {}", row, history_prediction);
            history_prediction = row[0] - history_prediction;
        }

        println!("{}\n", history_prediction);

        prediction += history_prediction;
    }

    println!("{}", prediction);
}

fn get_next(digits: &Vec<i64>) -> Vec<i64> {
    let mut next = vec![];
    let mut j = 1;
    for i in 0..digits.len() - 1 {
        let a = digits[i];
        let b = digits[j];

        let c = b - a;
        next.push(c);

        j += 1;
    }
    next
}
