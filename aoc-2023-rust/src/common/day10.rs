#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub coordinates: Coordinates,
    pub direction: Direction,
}

#[derive(Debug, Clone, Copy)]
pub enum Pipe {
    Vertical,
    Horizontal,
    BendL,
    BendJ,
    Bend7,
    BendF,
}

pub fn walk(state: State, map: &Vec<String>) -> Vec<State> {
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
