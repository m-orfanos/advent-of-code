use std::io::{self, BufRead};

use num_complex::Complex;

// I tried using a scanline fill algorithm but couldn't figure out the various edge cases
// TODO solve this problem using a flood-fill (BFS?) approach
fn main() {
    let mut vertices = vec![];
    vertices.push((0, 0));

    let mut perimeter = 0;

    let mut x_max = 0;
    let mut x_min = 0;
    let mut y_min = 0;
    let mut y_max = 0;
    let mut curr = Complex::new(0, 0);

    for input in io::stdin().lock().lines() {
        let line = input.unwrap();

        let vline: Vec<&str> = line.split(" ").map(|s| s.trim()).collect();
        let direction = vline[0].to_string();
        let mut nb_steps: i32 = vline[1].to_string().parse().unwrap();

        let walk = match direction.as_str() {
            "U" => Complex::new(-1, 0),
            "R" => Complex::new(0, 1),
            "D" => Complex::new(1, 0),
            "L" => Complex::new(0, -1),
            _ => panic!(),
        };

        perimeter += nb_steps;

        while nb_steps > 0 {
            curr = curr + walk;

            x_max = x_max.max(curr.re);
            x_min = x_min.min(curr.re);
            y_max = y_max.max(curr.im);
            y_min = y_min.min(curr.im);

            nb_steps -= 1;
        }

        vertices.push((curr.re, curr.im));
    }

    // use shoelace formula to calculate the area of the polygon
    // https://en.wikipedia.org/wiki/Shoelace_formula#Shoelace_formula
    let mut area = 0;
    for i in 0..vertices.len() - 1 {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[i + 1];
        area += x1 * y2 - x2 * y1;
    }
    area = i32::abs(area / 2);

    // use pick's theorem to determine the number of interior points
    // https://en.wikipedia.org/wiki/Pick%27s_theorem#Formula
    // A = i + b/2 - 1, rearranging
    // i = A - b/2 + 1

    // in this case b is the perimeter (p) of the polygon since
    // we only move U/R/D/L

    // the total number of trenches are is i + b, or
    // A -p/2 + 1 + p
    // A + p/2 + 1
    let ans = area + perimeter / 2 + 1;
    println!("{}", ans);
}
