use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    // parse input
    let mut grid = vec![];
    for input in io::stdin().lock().lines() {
        let line = input.unwrap();
        // do something with line

        let row: Vec<(String, u64, u64)> = line.chars().map(|ch| (ch.to_string(), 0, 0)).collect();
        grid.push(row);
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    enum Direction {
        North,
        East,
        South,
        West,
    }

    let directions_bitmap = HashMap::from([
        (Direction::North, 0b1),
        (Direction::East, 0b10),
        (Direction::South, 0b100),
        (Direction::West, 0b1000),
    ]);

    let mut paths = Vec::from([(0, 0, Direction::East)]);
    while paths.len() > 0 {
        let (x, y, d) = paths.pop().unwrap();
        let (tile, energized, visited_direction) = &mut grid[x][y];

        // avoids cycles
        // skip if tile has been already visited coming from the same direction
        if *energized == 1 && *visited_direction & directions_bitmap.get(&d).unwrap() > 0 {
            continue;
        }
        *visited_direction |= directions_bitmap.get(&d).unwrap();
        *energized = 1;

        // compute next path(s)
        let next_paths = if tile == "." {
            match d {
                Direction::North => vec![(-1, 0, Direction::North)],
                Direction::East => vec![(0, 1, Direction::East)],
                Direction::South => vec![(1, 0, Direction::South)],
                Direction::West => vec![(0, -1, Direction::West)],
            }
        } else if tile == "/" {
            match d {
                Direction::North => vec![(0, 1, Direction::East)],
                Direction::East => vec![(-1, 0, Direction::North)],
                Direction::South => vec![(0, -1, Direction::West)],
                Direction::West => vec![(1, 0, Direction::South)],
            }
        } else if tile == "\\" {
            match d {
                Direction::North => vec![(0, -1, Direction::West)],
                Direction::East => vec![(1, 0, Direction::South)],
                Direction::South => vec![(0, 1, Direction::East)],
                Direction::West => vec![(-1, 0, Direction::North)],
            }
        } else if tile == "|" {
            match d {
                Direction::North => vec![(-1, 0, Direction::North)],
                Direction::East => vec![(-1, 0, Direction::North), (1, 0, Direction::South)],
                Direction::South => vec![(1, 0, Direction::South)],
                Direction::West => vec![(-1, 0, Direction::North), (1, 0, Direction::South)],
            }
        } else if tile == "-" {
            match d {
                Direction::North => vec![(0, 1, Direction::East), (0, -1, Direction::West)],
                Direction::East => vec![(0, 1, Direction::East)],
                Direction::South => vec![(0, 1, Direction::East), (0, -1, Direction::West)],
                Direction::West => vec![(0, -1, Direction::West)],
            }
        } else {
            vec![]
        };

        // println!("{} {} {:?} {} {:?}", x, y, d, tile, next_paths);

        for (dx, dy, nextd) in next_paths {
            let xnext = (x as i32 + dx) as usize;
            let ynext = (y as i32 + dy) as usize;
            if xnext < grid.len() && ynext < grid[x].len() {
                paths.push((xnext, ynext, nextd));
            }
        }
    }

    let mut ans = 0;
    for row in grid {
        for (_tile, state, _) in row {
            if state == 1 {
                ans += 1;
                //     print!("#")
                // } else {
                //     print!(".");
            }
        }
        // println!("");
    }
    println!("{}", ans);
}
