use std::io::{self, BufRead};

use aoc::common::day09::evaluate_intermediary_states;

fn main() {
    let mut prediction = 0;

    for line_res in io::stdin().lock().lines() {
        let line = line_res.unwrap();

        // parse input
        let history: Vec<i64> = line.split(" ").map(|n| n.trim().parse().unwrap()).collect();

        let pyramid = evaluate_intermediary_states(history);

        // make prediction
        let mut history_prediction = 0;
        for row in pyramid.iter().rev() {
            // println!("{:?} {}", row, history_prediction);
            history_prediction = row[0] - history_prediction;
        }

        // println!("{}\n", history_prediction);

        prediction += history_prediction;
    }

    println!("{}", prediction);
}
