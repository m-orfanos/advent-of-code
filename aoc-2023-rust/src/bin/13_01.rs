use aoc::common::{intersection, transpose};
use std::io::{self, BufRead};

fn main() {
    // parse input
    let mut grids = vec![];
    let mut next_grid = vec![];
    for input in io::stdin().lock().lines() {
        let line = input.unwrap();
        if line.is_empty() {
            grids.push(next_grid);
            next_grid = vec![];
            continue;
        }
        next_grid.push(line);
    }
    grids.push(next_grid);

    // compute reflections
    let mut ans = 0;
    for grid in grids {
        let vertical_reflection = find_reflection(&grid);
        match vertical_reflection {
            Some(n) => {
                ans += n + 1;
            }
            None => (),
        }

        let grid2 = transpose2(grid);
        let horizontal_reflection = find_reflection(&grid2);
        match horizontal_reflection {
            Some(n) => {
                ans += 100 * (n + 1);
            }
            None => (),
        }
    }

    println!("{}", ans);
}

fn find_reflection(grid: &Vec<String>) -> Option<usize> {
    // the reflections MUST ALWAYS touch one
    // of the ends of the grid
    let mut reflections = vec![];

    for row in grid {
        let mut row_reflections = vec![];
        for i in 0..row.len() - 1 {
            // from the point of reflection (PR), i.e. (i, i+1)
            // create 2 strings of the same length
            // the length is the small distance from
            // the PR to the edge of either side of the map
            //
            // example 1
            //     PR
            // |---||---|
            // 01234567890123456789
            //
            // example 2
            //             PR
            //       |-----||-----|
            // 01234567890123456789
            let j = i + 1;

            let lhs = i;
            let rhs = row.len() - j - 1;
            let len = lhs.min(rhs);

            let a = &row[i - len..i + 1];
            let b = &row[j..j + len + 1];

            if a.to_string() == b.chars().rev().collect::<String>() {
                row_reflections.push((len, i));
            }
        }
        reflections.push(row_reflections);
    }

    let mut common = intersection(reflections);
    common.sort_by(|a, b| a.0.cmp(&b.0));
    match common.last() {
        Some(c) => Some(c.1),
        None => None,
    }
}

fn transpose2(grid: Vec<String>) -> Vec<String> {
    // splits Vec<String> into Vec<Vec<String>>
    // takes transpose
    // joins Vec<Vec<String>> -> Vec<String>
    to_str_grid(&transpose(&to_char_grid(&grid)))
}

fn to_str_grid(xss: &Vec<Vec<String>>) -> Vec<String> {
    xss.iter().map(|xs| xs.join("")).collect()
}

fn to_char_grid(grid: &Vec<String>) -> Vec<Vec<String>> {
    grid.iter()
        .map(|r| r.chars().map(|ch| ch.to_string()).collect())
        .collect()
}
