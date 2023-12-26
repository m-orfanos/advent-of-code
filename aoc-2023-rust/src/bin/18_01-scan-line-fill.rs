use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use num_complex::Complex;

fn main() {
    let (_, y_min, _, y_max, mut grid, _, mut edges) = parse_input();

    // scan-line fill algorithm
    let mut actives_edges = vec![];
    for y in y_min..y_max + 1 {
        while !edges.is_empty() && edges.last().unwrap().0 == y {
            actives_edges.push(edges.pop().unwrap());
        }

        actives_edges.sort_by(|a, b| a.1.cmp(&b.1)); // x of y_min

        // create list of x-coords to be filled
        // must determine all the various edge cases
        let mut xs = vec![];
        for ae in &actives_edges {
            if y == ae.4 {
                // intersects vertex with ymax
                xs.push(ae.1);
            }
            if y == ae.2 && y == ae.4 {
                // horizontal edge, add it again
                xs.push(ae.1);
            }
            if y > ae.2 && y < ae.4 {
                // intersects edge
                xs.push(ae.1);
            }
        }
        xs.sort();

        // fill-in the row
        for i in (0..xs.len()).step_by(2) {
            let x1 = xs[i];
            let x2 = xs[i + 1];
            for x in x1..x2 {
                let row = grid.entry(x).or_insert_with(|| HashMap::new());
                row.insert(y, "".to_string());
            }
        }

        // prep for next scan-line
        while !actives_edges.is_empty() {
            let e = actives_edges.pop().unwrap();
            if y == e.4 {
                // reached y_max
                continue;
            }
            edges.push((e.0 + 1, e.1, e.2, e.3, e.4, e.5, e.6));
        }
    }

    // display_grid(x_min, x_max, y_min, y_max, &grid);

    let mut ans = 0;
    for (_, ys) in grid {
        ans += ys.len();
    }
    println!("{}", ans);
}

// fn display_grid(
//     x_min: i32,
//     x_max: i32,
//     y_min: i32,
//     y_max: i32,
//     grid: &HashMap<i32, HashMap<i32, String>>,
// ) {
//     for y in (y_min..y_max + 1).rev() {
//         print!("{:<4} ", y);
//         for x in x_min..x_max + 1 {
//             if grid.get(&x).and_then(|r| r.get(&y)).is_some() {
//                 if x == 0 && y == 0 {
//                     print!("S");
//                 } else {
//                     print!("#");
//                 }
//             } else {
//                 print!(".");
//             }
//         }
//         println!("");
//     }
//     println!("");
// }

fn parse_input() -> (
    i32,
    i32,
    i32,
    i32,
    HashMap<i32, HashMap<i32, String>>,
    Vec<(i32, i32)>,
    Vec<(i32, i32, i32, i32, i32, f32, f32)>,
) {
    let mut grid: HashMap<i32, HashMap<i32, String>> = HashMap::new();
    let mut vertices = vec![];
    let mut edges = vec![];

    let mut x_max = 0;
    let mut x_min = 0;
    let mut y_min = 0;
    let mut y_max = 0;

    let mut curr = Complex::new(0, 0);

    for input in io::stdin().lock().lines() {
        let line = input.unwrap();

        let is: Vec<&str> = line.split(" ").map(|s| s.trim()).collect();
        let direction = is[0].to_string();
        let mut nb_steps: i32 = is[1].to_string().parse().unwrap();
        let color = is[2][1..8].to_string();

        let walk = match direction.as_str() {
            "U" => Complex::new(0, 1),
            "R" => Complex::new(1, 0),
            "D" => Complex::new(0, -1),
            "L" => Complex::new(-1, 0),
            _ => panic!(),
        };

        if curr.re == 0 && curr.im == 0 {
            let mut m = HashMap::new();
            m.insert(0, color.to_string());
            grid.insert(0, m);

            vertices.push((0, 0));
        }

        let start = curr;
        while nb_steps > 0 {
            curr = curr + walk;

            let row = grid.entry(curr.re).or_insert_with(|| HashMap::new());
            row.insert(curr.im, color.to_string());

            x_min = x_min.min(curr.re);
            y_min = y_min.min(curr.im);

            x_max = x_max.max(curr.re);
            y_max = y_max.max(curr.im);

            nb_steps -= 1;
        }

        let x_ymin;
        let y_min;
        let x_ymax;
        let y_max;
        if start.im < curr.im {
            x_ymin = start.re;
            y_min = start.im;

            x_ymax = curr.re;
            y_max = curr.im;
        } else {
            x_ymin = curr.re;
            y_min = curr.im;

            x_ymax = start.re;
            y_max = start.im;
        }

        let dir = curr - start;
        let slope = dir.im as f32 / dir.re as f32;
        let inv_slope = dir.re as f32 / dir.im as f32;

        vertices.push((curr.re, curr.im));
        edges.push((y_min, x_ymin, y_min, x_ymax, y_max, slope, inv_slope));
    }

    vertices.pop();
    edges.sort_by(|a, b| b.0.cmp(&a.0));

    (x_min, y_min, x_max, y_max, grid, vertices, edges)
}
