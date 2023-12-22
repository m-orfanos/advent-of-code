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
    consecutive: i8,
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

fn get_neighbors(node: &Node, rows: usize, cols: usize) -> Vec<Node> {
    let mut ans = vec![];
    if node.x > 0 && node.direction != Direction::South {
        let consecutive = get_consecutive(node, Direction::North);
        if consecutive >= 0 {
            ans.push(Node {
                x: node.x - 1,
                y: node.y,
                direction: Direction::North,
                consecutive,
            });
        }
    }
    if node.y > 0 && node.direction != Direction::East {
        let consecutive = get_consecutive(node, Direction::West);
        if consecutive >= 0 {
            ans.push(Node {
                x: node.x,
                y: node.y - 1,
                direction: Direction::West,
                consecutive,
            });
        }
    }
    if node.x < rows - 1 && node.direction != Direction::North {
        let consecutive = get_consecutive(node, Direction::South);
        if consecutive >= 0 {
            ans.push(Node {
                x: node.x + 1,
                y: node.y,
                direction: Direction::South,
                consecutive,
            });
        }
    }
    if node.y < cols - 1 && node.direction != Direction::West {
        let consecutive = get_consecutive(node, Direction::East);
        if consecutive >= 0 {
            ans.push(Node {
                x: node.x,
                y: node.y + 1,
                direction: Direction::East,
                consecutive,
            });
        }
    }
    ans
}

fn get_consecutive(node: &Node, direction: Direction) -> i8 {
    if node.direction == direction {
        // zero-indexed
        if node.consecutive == 2 {
            return -1;
        } else {
            return node.consecutive + 1;
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
    };
    let start_f_score = h(&grid[start.x][start.y], &goal);
    open_nodes.push(Reverse((start_f_score, start_node)));

    let mut came_from = HashMap::new();
    let mut neighbor_nodes = HashMap::new();

    // tracks heat loss
    let mut g_scores = HashMap::new();
    g_scores.insert(start_node, 0);

    // walk across map
    while !open_nodes.is_empty() {
        // retrieve lowest f-score
        let Reverse((_, curr)) = open_nodes.pop().unwrap();

        if curr.x == goal.x && curr.y == goal.y {
            // reached the goal
            return *g_scores.get(&curr).unwrap();
        }

        // visit neighbors
        let neighbors = neighbor_nodes
            .entry(curr)
            .or_insert_with(|| get_neighbors(&curr, rows, cols));
        for n in neighbors {
            let tmp_g_score = g_scores[&curr] + d(&grid[n.x][n.y]);
            let g_score_neighbor = g_scores.get(n).unwrap_or(&i32::MAX);
            if tmp_g_score < *g_score_neighbor {
                // found better path
                came_from.insert(*n, curr);
                g_scores.insert(*n, tmp_g_score);
                open_nodes.push(Reverse((tmp_g_score + h(&grid[n.x][n.y], &goal), *n)));
            }
        }
    }

    // no path found from start to goal
    -1
}
