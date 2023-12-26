use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use num_complex::Complex;

struct Vertex {
    x: i32,
    y: i32,
}

struct Edge {
    v1: Vertex,
    v2: Vertex,
    y_min: i32,
}

// scan-line fill approach
// https://en.wikipedia.org/wiki/Flood_fill#Span_filling
// https://www.educative.io/answers/what-is-scanline-fill-algorithm
// https://gist.github.com/esmitt/17d36f23c6e98f0b1fa5bda06490d9c4
fn main() {
    let (y_min, y_max, mut grid, mut edges) = parse_input();

    let mut actives_edges = vec![];

    for y in y_min..y_max + 1 {
        // important! edges is reverse sorted (desc) by ymin
        while !edges.is_empty() && edges.last().unwrap().y_min == y {
            actives_edges.push(edges.pop().unwrap());
        }

        // now sort active edges by x of ymin (first vertex)
        actives_edges.sort_by(|a, b| a.v1.x.cmp(&b.v1.x));

        // create list of x-coords to be filled
        // must determine all the various edge cases
        let mut xs = vec![];
        for ae in &actives_edges {
            if y == ae.v2.y {
                // intersects vertex with ymax
                xs.push(ae.v1.x);
            }
            if y == ae.v1.y && y == ae.v2.y {
                // horizontal edge, add it again
                xs.push(ae.v1.x);
            }
            if y > ae.v1.y && y < ae.v2.y {
                // intersects edge
                xs.push(ae.v1.x);
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
            if y == e.v2.y {
                // reached y_max, drop it going forward
                continue;
            }
            edges.push(Edge {
                v1: e.v1,
                v2: e.v2,
                y_min: e.y_min + 1,
            });
        }
    }

    // count number of trenches
    let mut ans = 0;
    for (_, ys) in grid {
        ans += ys.len();
    }
    println!("{}", ans);
}

fn parse_input() -> (i32, i32, HashMap<i32, HashMap<i32, String>>, Vec<Edge>) {
    let mut grid: HashMap<i32, HashMap<i32, String>> = HashMap::new();
    // let mut vertices = vec![];
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

            // vertices.push(Vertex { x: 0, y: 0 });
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

        let v1;
        let v2;
        if start.im < curr.im {
            v1 = Vertex {
                x: start.re,
                y: start.im,
            };
            v2 = Vertex {
                x: curr.re,
                y: curr.im,
            };
        } else {
            v1 = Vertex {
                x: curr.re,
                y: curr.im,
            };
            v2 = Vertex {
                x: start.re,
                y: start.im,
            };
        }

        // vertices.push(Vertex {
        //     x: curr.re,
        //     y: curr.im,
        // });
        edges.push(Edge { v1, v2, y_min });
    }

    // the last vertex is the same as the start, remove it
    // vertices.pop();
    edges.sort_by(|a, b| b.y_min.cmp(&a.y_min));

    (y_min, y_max, grid, edges)
}
