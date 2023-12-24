use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use num_complex::Complex;

fn main() {
    let mut grid: HashMap<i32, HashMap<i32, String>> = HashMap::new();

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
            "U" => Complex::new(-1, 0),
            "R" => Complex::new(0, 1),
            "D" => Complex::new(1, 0),
            "L" => Complex::new(0, -1),
            _ => panic!(),
        };

        if curr.re == 0 && curr.im == 0 {
            let mut m = HashMap::new();
            m.insert(0, color.to_string());
            grid.insert(0, m);
        }

        while nb_steps > 0 {
            curr = curr + walk;

            let row = grid.entry(curr.re).or_insert_with(|| HashMap::new());
            row.insert(curr.im, color.to_string());

            x_max = x_max.max(curr.re);
            x_min = x_min.min(curr.re);
            y_max = y_max.max(curr.im);
            y_min = y_min.min(curr.im);

            nb_steps -= 1;
        }
    }

    // display dig plan
    for x in x_min..x_max + 1 {
        for y in y_min..y_max + 1 {
            if grid.get(&x).and_then(|r| r.get(&y)).is_some() {
                if x == 0 && y == 0 {
                    print!("S");
                } else {
                    print!("#");
                }
            } else {
                print!(".");
            }
        }
        println!("");
    }

    println!("({},{}) ({},{})", x_min, y_min, x_max, y_max);
}
