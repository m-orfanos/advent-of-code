use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    // parse input
    let mut grid = vec![];
    for input in io::stdin().lock().lines() {
        let line = input.unwrap();
        let row: Vec<(String, u64)> = line.chars().map(|ch| (ch.to_string(), 0)).collect();
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

    let energized = 0b10000;

    // walk along the grid
    let mut paths = Vec::from([(0, 0, Direction::East)]);
    while paths.len() > 0 {
        let (x, y, d) = paths.pop().unwrap();
        let (tile, state) = &mut grid[x][y];

        // avoids cycles
        // skip if tile has been already visited coming from the same direction
        if *state & energized > 0 && *state & directions_bitmap.get(&d).unwrap() > 0 {
            continue;
        }
        *state |= directions_bitmap.get(&d).unwrap() | energized;

        // compute next direction(s)
        let next_direction = if tile == "." {
            match d {
                Direction::North => vec![Direction::North],
                Direction::East => vec![Direction::East],
                Direction::South => vec![Direction::South],
                Direction::West => vec![Direction::West],
            }
        } else if tile == "/" {
            match d {
                Direction::North => vec![Direction::East],
                Direction::East => vec![Direction::North],
                Direction::South => vec![Direction::West],
                Direction::West => vec![Direction::South],
            }
        } else if tile == "\\" {
            match d {
                Direction::North => vec![Direction::West],
                Direction::East => vec![Direction::South],
                Direction::South => vec![Direction::East],
                Direction::West => vec![Direction::North],
            }
        } else if tile == "|" {
            match d {
                Direction::North => vec![Direction::North],
                Direction::East => vec![Direction::North, Direction::South],
                Direction::South => vec![Direction::South],
                Direction::West => vec![Direction::North, Direction::South],
            }
        } else if tile == "-" {
            match d {
                Direction::North => vec![Direction::East, Direction::West],
                Direction::East => vec![Direction::East],
                Direction::South => vec![Direction::East, Direction::West],
                Direction::West => vec![Direction::West],
            }
        } else {
            vec![]
        };

        for next_direction in next_direction {
            let dx = match next_direction {
                Direction::North => -1,
                Direction::East => 0,
                Direction::South => 1,
                Direction::West => 0,
            };
            let dy = match next_direction {
                Direction::North => 0,
                Direction::East => 1,
                Direction::South => 0,
                Direction::West => -1,
            };
            // compute next (x,y) position on map
            let xnext = (x as i32 + dx) as usize;
            let ynext = (y as i32 + dy) as usize;

            // don't fall off the map
            if xnext < grid.len() && ynext < grid[x].len() {
                paths.push((xnext, ynext, next_direction));
            }
        }
    }

    let mut ans = 0;
    for row in grid {
        for (_, state) in row {
            if state & energized > 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
