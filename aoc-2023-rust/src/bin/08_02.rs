use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use aoc::common::lcm;

fn main() {
    // parse input
    let mut it = io::stdin().lock().lines();
    let instructions = it.next().unwrap().unwrap();
    it.next(); // blank line

    let mut map: HashMap<String, [String; 2]> = HashMap::new();
    for input in it {
        let line = input.unwrap().to_string();
        let parts: Vec<_> = line.split("=").collect();
        let key = parts[0].trim().to_string();

        let nodes: Vec<_> = parts[1].split(",").collect();
        let left = nodes[0].replace("(", "").trim().to_string();
        let right = nodes[1].replace(")", "").trim().to_string();
        map.insert(key, [left, right]);
    }

    // find starting nodes
    let mut curr_nodes = Vec::new();
    for k in map.keys() {
        if k.ends_with("A") {
            curr_nodes.push(k);
        }
    }
    curr_nodes.sort();

    // traverse each starting node until a match is found (e.g. ends with z)
    // starting from that point, calculate the number of steps until the matched
    // node is reached again (every path is a closed loop; e.g. eventually repeats itself)
    let mut path_lengths = vec![];
    for j in 0..curr_nodes.len() {
        let mut cnt = 0;
        let mut cnt2 = 0;
        let mut instruction_counter = 0;
        let mut ends_with_z = false;
        while !ends_with_z {
            cnt += 1;
            ends_with_z = false;

            if instruction_counter >= instructions.len() {
                instruction_counter = 0;
            }

            let instruction = instructions.chars().nth(instruction_counter).unwrap();
            let index = if instruction == 'L' { 0 } else { 1 };

            let mut next_nodes = Vec::new();
            for (k, curr) in curr_nodes.iter().enumerate() {
                let nodes: &[String; 2] = &map[*curr];
                let next = &nodes[index];
                if next.ends_with("Z") && j == k {
                    if cnt2 <= 0 {
                        cnt = 0;
                        cnt2 += 1;
                    } else {
                        ends_with_z = true;
                    }
                }
                next_nodes.push(next);
            }

            curr_nodes = next_nodes;
            instruction_counter += 1;
        }
        path_lengths.push(cnt);
    }

    // here are the results for the computation above
    // ["AAA", "DPA", "GTA", "QLA", "VJA", "XQA"]
    // 0->12361
    // 1->20777
    // 2->16043
    // 3->19199
    // 4->18673
    // 5->15517
    //
    // the first path repeats itself after 12,361 steps and the second after 20,777
    //
    // when will these two paths intersect?
    // for the first path it takes 79 cycles and the second path its 47 cycles
    // 79*12,361 == 47*20,777 = 976,519
    //
    // (Note how the numbers 47 and 79 are special... they are all primes. This is a clue.)
    //
    // using a smaller example
    // 0->2
    // 1->3
    //
    //  steps 123456
    // path 0 ->->->|
    // path 1 -->-->|
    // where `-` denotes a step and `>` a match/destination
    //
    // using a slightly larger example
    // 0->14 (2*7)
    // 1->16 (2*2*2*2)
    // calculate the prime factors, take the common factors and multiply them together
    // 2*2*2*2*7=112
    //                10         20        30        40        50        60        70        80        90        100       110
    //        ---------|---------|---------|---------|---------|---------|---------|---------|---------|---------|---------|
    //        0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678912
    // path 0 ------------->------------->------------->------------->------------->------------->------------->------------->|
    // path 1 --------------->--------------->--------------->--------------->--------------->--------------->--------------->|
    // (it's 112 steps)
    //
    // we've essentially calculated the lowest-common-multiple of two numbers
    // now we need to do it for all paths
    let ans = lcm(path_lengths);
    println!("{}", ans);
}
