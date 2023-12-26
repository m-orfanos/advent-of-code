use std::io::{self, BufRead};

use aoc::common::day10::{walk, Coordinates, Direction, State};

fn main() {
    let mut start_x = 0;
    let mut start_y = 0;
    let mut map: Vec<String> = Vec::new();
    for (y, line_res) in io::stdin().lock().lines().enumerate() {
        let line = line_res.unwrap();
        if line.contains('S') {
            start_x = line.find('S').unwrap();
            start_y = y.into();
        }
        map.push(line.trim().to_string());
    }

    // println!("Starting position: ({},{})", start_x, start_y);

    // There are only 2 valid starting states, but
    // it's impossible to know without checking.
    // Instead take a "walk" in any of the 4
    // directions until you hit an "invalid"
    // state (e.g. a wall, off map, starting position)
    let starting_states = [
        State {
            coordinates: Coordinates {
                x: start_x as i32,
                y: start_y as i32 - 1,
            },
            direction: Direction::North,
        },
        State {
            coordinates: Coordinates {
                x: start_x as i32 + 1,
                y: start_y as i32,
            },
            direction: Direction::East,
        },
        State {
            coordinates: Coordinates {
                x: start_x as i32 - 1,
                y: start_y as i32,
            },
            direction: Direction::West,
        },
        State {
            coordinates: Coordinates {
                x: start_x as i32,
                y: start_y as i32 + 1,
            },
            direction: Direction::South,
        },
    ];

    let mut ans = 0;
    for state in starting_states {
        // println!("Start: {:?}", state);
        let path = walk(state, &map);
        // println!("NB Steps: {}", path.len());
        // println!("");
        ans = ans.max(path.len());
    }

    // this is the length of the entire cycle
    // we only want the half
    println!("{}", ans / 2);
}
