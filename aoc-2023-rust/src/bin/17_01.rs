use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

#[derive(Debug)]
struct Tile {
    x: usize,
    y: usize,
    cost: i32,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
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

fn find_min_f_score(open_nodes: &HashSet<Node>, f_scores: &HashMap<Node, i32>) -> Node {
    let mut ns = vec![];
    for n in open_nodes {
        let score = f_scores[n];
        ns.push((score, n.x, n.y, n.direction, n.consecutive));
    }
    ns.sort_by(|a, b| a.0.cmp(&b.0));
    Node {
        x: ns[0].1,
        y: ns[0].2,
        direction: ns[0].3,
        consecutive: ns[0].4,
    }
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

    let mut open_nodes = HashSet::new();
    let start_node = Node {
        x: start.x,
        y: start.y,
        direction: Direction::East,
        consecutive: 0,
    };
    open_nodes.insert(start_node);

    let mut came_from = HashMap::new();
    let mut neighbor_nodes = HashMap::new();

    let mut g_scores = HashMap::new();
    g_scores.insert(start_node, 0);

    let mut f_scores = HashMap::new();
    f_scores.insert(start_node, h(&grid[start.x][start.y], &goal));

    while !open_nodes.is_empty() {
        let curr = find_min_f_score(&open_nodes, &f_scores);
        if curr.x == goal.x && curr.y == goal.y {
            return *g_scores.get(&curr).unwrap();
        }

        open_nodes.remove(&curr);

        let neighbors = neighbor_nodes
            .entry(curr)
            .or_insert_with(|| get_neighbors(&curr, rows, cols));
        for n in neighbors {
            // "distance" from start to the neighbor through current
            let tmp_g_score = g_scores[&curr] + d(&grid[n.x][n.y]);
            let g_score_neighbor = g_scores.get(n).unwrap_or(&i32::MAX);
            if tmp_g_score < *g_score_neighbor {
                // found better path
                came_from.insert(*n, curr);
                g_scores.insert(*n, tmp_g_score);
                f_scores.insert(*n, tmp_g_score + h(&grid[n.x][n.y], &goal));
                if !open_nodes.contains(&n) {
                    open_nodes.insert(*n);
                }
            }
        }
    }

    // no path found from start to goal
    -1
}
