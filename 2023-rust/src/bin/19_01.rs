use std::{
    collections::BTreeMap,
    io::{self, BufRead},
};

use regex::Regex;

fn main() {
    // parse input
    let input_regex = Regex::new(r"(.*)\{(.[<>]\d+:.+),(.*)\}").unwrap();
    let rules_regex = Regex::new(r"(.*)([<>])(.*):(.*)").unwrap();
    let parts_regex = Regex::new(r"\{x=(.*),m=(.*),a=(.*),s=(.*)\}").unwrap();

    let mut workflows = BTreeMap::new();
    let mut parts = vec![];

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
            parts.push(part);
        }
    }

    // solve problem
    let mut ans: u32 = 0;
    for part in &parts {
        let mut curr = "in";
        while workflows.contains_key(curr) {
            let (rules, default_next) = workflows.get(curr).unwrap();
            let mut is_found = false;
            for (part_name, op, cnt, next) in rules {
                let pnb = part.get(part_name).unwrap();
                if (op == "<" && pnb < cnt) || (op == ">" && pnb > cnt) {
                    curr = next;
                    is_found = true;
                    break;
                }
            }
            if !is_found {
                curr = default_next;
            }
        }

        if curr == "A" {
            ans += part.values().sum::<u32>();
        }
    }
    println!("{}", ans);
}

fn parse_part(line: String, parts_regex: &Regex) -> BTreeMap<String, u32> {
    let cap_parts = parts_regex.captures(&line).unwrap();

    fn parse_part(cap_parts: &regex::Captures<'_>, i: usize) -> u32 {
        let x: u32 = cap_parts
            .get(i)
            .unwrap()
            .as_str()
            .to_string()
            .parse()
            .unwrap();
        x
    }

    BTreeMap::from([
        ("x".to_string(), parse_part(&cap_parts, 1)),
        ("m".to_string(), parse_part(&cap_parts, 2)),
        ("a".to_string(), parse_part(&cap_parts, 3)),
        ("s".to_string(), parse_part(&cap_parts, 4)),
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
