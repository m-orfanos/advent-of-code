use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    io::{self, BufRead},
};

#[derive(Debug)]
struct Tile {
    x: usize,
    y: usize,
    cost: i32,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, PartialOrd, Ord)]
struct Node {
    x: usize,
    y: usize,
    direction: Direction,
    consecutive: i32,
    g_score: Option<i32>,
}

fn main() {
    let mut grid = vec![];
    for (i, input) in io::stdin().lock().lines().enumerate() {
        let line = input.unwrap();
        let mut row = vec![];
        for (j, ch) in line.chars().enumerate() {
            row.push(Tile {
                x: i,
                y: j,
                cost: ch.to_string().parse().unwrap(),
            })
        }
        grid.push(row);
    }
    let heat_loss = a_star(&grid, &grid[0][0], &grid[grid.len() - 1][grid[0].len() - 1]);
    println!("{}", heat_loss);
}

fn h(neighbor: &Tile, goal: &Tile) -> i32 {
    // manhattan distance
    neighbor.x.abs_diff(goal.x) as i32 + neighbor.y.abs_diff(goal.y) as i32
}

fn d(neighbor: &Tile) -> i32 {
    neighbor.cost
}

fn get_neighbors(
    x: usize,
    y: usize,
    direction: Direction,
    consecutive: i32,
    rows: usize,
    cols: usize,
) -> Vec<Node> {
    let mut ans = vec![];
    if x > 0 && direction != Direction::South {
        let nc = get_next_consecutive(direction, consecutive, Direction::North);
        if nc >= 0 {
            ans.push(Node {
                x: x - 1,
                y,
                direction: Direction::North,
                consecutive: nc,
                g_score: None,
            });
        }
    }
    if y > 0 && direction != Direction::East {
        let nc = get_next_consecutive(direction, consecutive, Direction::West);
        if nc >= 0 {
            ans.push(Node {
                x,
                y: y - 1,
                direction: Direction::West,
                consecutive: nc,
                g_score: None,
            });
        }
    }
    if x < rows - 1 && direction != Direction::North {
        let nc = get_next_consecutive(direction, consecutive, Direction::South);
        if nc >= 0 {
            ans.push(Node {
                x: x + 1,
                y,
                direction: Direction::South,
                consecutive: nc,
                g_score: None,
            });
        }
    }
    if y < cols - 1 && direction != Direction::West {
        let nc = get_next_consecutive(direction, consecutive, Direction::East);
        if nc >= 0 {
            ans.push(Node {
                x,
                y: y + 1,
                direction: Direction::East,
                consecutive: nc,
                g_score: None,
            });
        }
    }
    ans
}

fn get_next_consecutive(direction: Direction, consecutive: i32, next_direction: Direction) -> i32 {
    if direction == next_direction {
        // zero-indexed
        if consecutive == 2 {
            return -1;
        } else {
            return consecutive + 1;
        }
    }
    0
}

fn a_star(grid: &Vec<Vec<Tile>>, start: &Tile, goal: &Tile) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();

    // stores "f-scores", i.e. heat loss estimates
    // this is a max-heap, need to use inverse
    let mut open_nodes = BinaryHeap::with_capacity(1e6 as usize);

    // note the difference with the "classic" A-star algorithm,
    // there are 4 parameters instead of only coordinates
    let start_node = Node {
        x: start.x,
        y: start.y,
        direction: Direction::East,
        consecutive: 0,
        g_score: Some(0),
    };
    let start_f_score = h(&grid[start.x][start.y], &goal);
    open_nodes.push(Reverse((start_f_score, start_node)));

    let mut neighbor_nodes = HashMap::new();

    // walk across map
    while !open_nodes.is_empty() {
        // retrieve lowest f-score
        let Reverse((_, curr)) = open_nodes.pop().unwrap();

        if curr.x == goal.x && curr.y == goal.y {
            // reached the goal
            return curr.g_score.unwrap();
        }

        // visit neighbors
        let neighbors = neighbor_nodes
            .entry((curr.x, curr.y, curr.direction, curr.consecutive))
            .or_insert_with(|| {
                get_neighbors(curr.x, curr.y, curr.direction, curr.consecutive, rows, cols)
            });
        for n in neighbors {
            let tmp_g_score = curr.g_score.unwrap() + d(&grid[n.x][n.y]);
            let g_score_neighbor = n.g_score.unwrap_or(i32::MAX);
            if tmp_g_score < g_score_neighbor {
                // found better path
                n.g_score = Some(tmp_g_score);
                open_nodes.push(Reverse((tmp_g_score + h(&grid[n.x][n.y], &goal), *n)));
            }
        }
    }

    // no path found from start to goal
    -1
}
