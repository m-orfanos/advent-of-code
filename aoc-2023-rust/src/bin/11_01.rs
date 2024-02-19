use std::io::{self, BufRead};

fn main() {
    let mut grid: Vec<Vec<String>> = Vec::new();

    // parse input
    for input in io::stdin().lock().lines() {
        let row: Vec<String> = input
            .unwrap()
            .trim()
            .chars()
            .map(|s| s.to_string())
            .collect();

        // expand rows
        let has_galaxy = row.iter().any(|ch| ch.contains("#"));
        if !has_galaxy {
            grid.push(row.clone());
        }

        grid.push(row);
    }

    // expand cols, starting from the end
    let mut cols = vec![];
    let mut j = grid[0].len() - 1;
    while j > 0 {
        let mut has_galaxy = false;
        for row in &grid {
            if row[j] == "#" {
                has_galaxy = true;
                break;
            }
        }
        if !has_galaxy {
            cols.push(j);
            for row in grid.iter_mut() {
                row.insert(j, ".".to_string());
            }
        }
        j -= 1;
    }

    // calculate positions of every galaxy
    let mut galaxies = vec![];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == "#" {
                galaxies.push((i, j));
            }
        }
    }

    // calculate distance
    let mut distance = 0;
    for g1 in 0..galaxies.len() {
        let (x1, y1) = galaxies[g1];
        for g2 in (g1 + 1)..galaxies.len() {
            let (x2, y2) = galaxies[g2];
            distance += y2.abs_diff(y1) + x2.abs_diff(x1);
        }
    }

    println!("{}", distance);
}
