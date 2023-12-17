use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
struct Coordinates {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct State {
    coordinates: Coordinates,
    direction: Direction,
}

#[derive(Debug, Clone, Copy)]
enum Pipe {
    Vertical,
    Horizontal,
    BendL,
    BendJ,
    Bend7,
    BendF,
}

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
    // state (e.g. a wall, off map, starting line)
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

fn walk(state: State, map: &Vec<String>) -> Vec<State> {
    // given a starting position, walk
    // along the map as far as possible
    let mut path = vec![];
    path.push(state);

    loop {
        let curr = path[path.len() - 1];
        let next = get_next(&map, &curr);
        match next {
            Some(n) => path.push(n),
            None => break,
        }
    }
    path
}

fn as_pipe(ch: char) -> Option<Pipe> {
    // only care about valid routes
    match ch {
        '|' => Some(Pipe::Vertical),
        '-' => Some(Pipe::Horizontal),
        'L' => Some(Pipe::BendL),
        'J' => Some(Pipe::BendJ),
        '7' => Some(Pipe::Bend7),
        'F' => Some(Pipe::BendF),
        _ => None, // includes S, the starting position
    }
}

fn next_state(state: &State, pipe: &Pipe) -> Option<State> {
    let ans = match (state.direction, pipe) {
        // NORTH
        (Direction::North, Pipe::Vertical) => Some(State {
            coordinates: Coordinates {
                x: state.coordinates.x,
                y: state.coordinates.y - 1,
            },
            direction: Direction::North,
        }),
        (Direction::North, Pipe::Horizontal) => None,
        (Direction::North, Pipe::BendL) => None,
        (Direction::North, Pipe::BendJ) => None,
        (Direction::North, Pipe::Bend7) => Some(State {
            coordinates: Coordinates {
                x: state.coordinates.x - 1,
                y: state.coordinates.y,
            },
            direction: Direction::West,
        }),
        (Direction::North, Pipe::BendF) => Some(State {
            coordinates: Coordinates {
                x: state.coordinates.x + 1,
                y: state.coordinates.y,
            },
            direction: Direction::East,
        }),
        // EAST
        (Direction::East, Pipe::Vertical) => None,
        (Direction::East, Pipe::Horizontal) => Some(State {
            coordinates: Coordinates {
                x: state.coordinates.x + 1,
                y: state.coordinates.y,
            },
            direction: Direction::East,
        }),
        (Direction::East, Pipe::BendL) => None,
        (Direction::East, Pipe::BendJ) => Some(State {
            coordinates: Coordinates {
                x: state.coordinates.x,
                y: state.coordinates.y - 1,
            },
            direction: Direction::North,
        }),
        (Direction::East, Pipe::Bend7) => Some(State {
            coordinates: Coordinates {
                x: state.coordinates.x,
                y: state.coordinates.y + 1,
            },
            direction: Direction::South,
        }),
        (Direction::East, Pipe::BendF) => None,
        // SOUTH
        (Direction::South, Pipe::Vertical) => Some(State {
            coordinates: Coordinates {
                x: state.coordinates.x,
                y: state.coordinates.y + 1,
            },
            direction: Direction::South,
        }),
        (Direction::South, Pipe::Horizontal) => None,
        (Direction::South, Pipe::BendL) => Some(State {
            coordinates: Coordinates {
                x: state.coordinates.x + 1,
                y: state.coordinates.y,
            },
            direction: Direction::East,
        }),
        (Direction::South, Pipe::BendJ) => Some(State {
            coordinates: Coordinates {
                x: state.coordinates.x - 1,
                y: state.coordinates.y,
            },
            direction: Direction::West,
        }),
        (Direction::South, Pipe::Bend7) => None,
        (Direction::South, Pipe::BendF) => None,
        // WEST
        (Direction::West, Pipe::Vertical) => None,
        (Direction::West, Pipe::Horizontal) => Some(State {
            coordinates: Coordinates {
                x: state.coordinates.x - 1,
                y: state.coordinates.y,
            },
            direction: Direction::West,
        }),
        (Direction::West, Pipe::BendL) => Some(State {
            coordinates: Coordinates {
                x: state.coordinates.x,
                y: state.coordinates.y - 1,
            },
            direction: Direction::North,
        }),
        (Direction::West, Pipe::BendJ) => None,
        (Direction::West, Pipe::Bend7) => None,
        (Direction::West, Pipe::BendF) => Some(State {
            coordinates: Coordinates {
                x: state.coordinates.x,
                y: state.coordinates.y + 1,
            },
            direction: Direction::South,
        }),
    };

    // println!("{:?}", ans);
    ans
}

fn get_next(map: &Vec<String>, state: &State) -> Option<State> {
    map.get(state.coordinates.y as usize)
        .and_then(|y| y.chars().nth(state.coordinates.x as usize))
        .and_then(|ch| as_pipe(ch))
        .and_then(|p| next_state(state, &p))
}
