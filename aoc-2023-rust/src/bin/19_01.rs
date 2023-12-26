use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use regex::Regex;

fn main() {
    let input_regex = Regex::new(r"(.*)\{(.[<>]\d+:.+),(.*)\}").unwrap();
    let rules_regex = Regex::new(r"(.*)([<>])(.*):(.*)").unwrap();
    let parts_regex = Regex::new(r"\{x=(.*),m=(.*),a=(.*),s=(.*)\}").unwrap();

    let mut workflows = HashMap::new();
    let mut parts = HashMap::new();

    let mut section = 0;
    for input in io::stdin().lock().lines() {
        let line = input.unwrap();

        if line.is_empty() {
            section += 1;
            continue;
        }

        if section == 0 {
            let workflow = parse_workflows(line, &input_regex, &rules_regex);
            workflows.insert(workflow.0, (workflow.1, workflow.2));
        } else {
            let part = parse_part(line, &parts_regex);
            parts.insert(part["x"], part);
        }
    }

    println!("Workflows");
    for w in workflows {
        println!("{:?}", w);
    }

    println!("Parts");
    for p in parts {
        println!("{:?}", p);
    }
}

fn parse_part(line: String, parts_regex: &Regex) -> HashMap<&str, i32> {
    let cap_parts = parts_regex.captures(&line).unwrap();

    fn parse_part(cap_parts: &regex::Captures<'_>, i: usize) -> i32 {
        let x: i32 = cap_parts
            .get(i)
            .unwrap()
            .as_str()
            .to_string()
            .parse()
            .unwrap();
        x
    }

    HashMap::from([
        ("x", parse_part(&cap_parts, 1)),
        ("m", parse_part(&cap_parts, 2)),
        ("a", parse_part(&cap_parts, 3)),
        ("s", parse_part(&cap_parts, 4)),
    ])
}

fn parse_workflows(
    line: String,
    input_regex: &Regex,
    rules_regex: &Regex,
) -> (String, Vec<(String, String, u32, String)>, String) {
    let cap_line = input_regex.captures(&line).unwrap();
    let name = cap_line.get(1).unwrap().as_str().to_string();
    let rules: Vec<(String, String, u32, String)> = cap_line
        .get(2)
        .unwrap()
        .as_str()
        .split(",")
        .map(|r| {
            let cap_rules = rules_regex.captures(r).unwrap();
            let part = cap_rules.get(1).unwrap().as_str().to_string();
            let op = cap_rules.get(2).unwrap().as_str().to_string();
            let cnt: u32 = cap_rules.get(3).unwrap().as_str().parse().unwrap();
            let next = cap_rules.get(4).unwrap().as_str().to_string();
            (part, op, cnt, next)
        })
        .collect();
    let next = cap_line.get(3).unwrap().as_str().to_string();
    (name, rules, next)
}
