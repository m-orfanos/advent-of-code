use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
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

    let mut curr_nodes = Vec::new();
    for k in map.keys() {
        if k.ends_with("A") {
            curr_nodes.push(k);
        }
    }

    let mut instruction_counter = 0;
    let mut cnt = 0;
    let mut ends_with_z = false;
    while !ends_with_z {
        ends_with_z = true;
        if instruction_counter >= instructions.len() {
            instruction_counter = 0;
        }

        let instruction = instructions.chars().nth(instruction_counter).unwrap();
        let index = if instruction == 'L' { 0 } else { 1 };

        let mut next_nodes = Vec::new();
        for curr in curr_nodes.iter() {
            let nodes: &[String; 2] = &map[*curr];
            let next = &nodes[index];
            if !next.ends_with("Z") {
                ends_with_z = false;
            }
            next_nodes.push(next);
        }

        println!("{} {} {} {:?}->{:?}", cnt, instruction_counter, instruction, curr_nodes, next_nodes);

        curr_nodes = next_nodes;
        cnt += 1;
        instruction_counter += 1;
    }
    println!("{}", cnt);
}
