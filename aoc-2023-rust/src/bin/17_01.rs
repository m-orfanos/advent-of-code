use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    io::{self, BufRead},
};

use num_complex::Complex;

struct Tile {
    x: usize,
    y: usize,
    cost: i32,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    xy: Complex<i32>,
    x: usize,
    y: usize,
    direction: Complex<i32>,
    consecutive: i32,
    g_score: Option<i32>,
    f_score: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f_score.cmp(&other.f_score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.f_score.partial_cmp(&other.f_score)
    }
}

const ROTATE_CLOCKWISE: Complex<i32> = Complex::new(0, 1);
const ROTATE_ANTI_CLOCKWISE: Complex<i32> = Complex::new(0, -1);

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
    (neighbor.x.abs_diff(goal.x) + neighbor.y.abs_diff(goal.y)) as i32
}

fn d(neighbor: &Tile) -> i32 {
    neighbor.cost
}

fn get_neighbors(
    pos: Complex<i32>,
    dir: Complex<i32>,
    consecutive: i32,
    rows: i32,
    cols: i32,
) -> Vec<Node> {
    fn maybe_push(
        p: Complex<i32>,
        d: Complex<i32>,
        c: i32,
        v: &mut Vec<Node>,
        rows: i32,
        cols: i32,
    ) {
        if 0 <= p.re && p.re < rows && 0 <= p.im && p.im < cols {
            v.push(Node {
                xy: p,
                x: p.re as usize,
                y: p.im as usize,
                direction: d,
                consecutive: c,
                g_score: None,
                f_score: 0,
            });
        }
    }

    // check neighbors
    // there are 4 neighbors, but we only care about at most 3 of them
    // 1) ignore if moving in the opposite direction we're coming from
    // 2) ignore if moving in the same direction 3 times
    // 3) ignore falling off map
    //
    // we model the coordinates and direction as complex numbers to
    // take advantage of complex arithmetic

    let mut ans = vec![];

    // same direction
    let dir1 = dir;
    let pos1 = pos + dir1;
    if consecutive < 2 {
        maybe_push(pos1, dir1, consecutive + 1, &mut ans, rows, cols);
    }

    // direction 90d
    let dir2 = dir * ROTATE_CLOCKWISE;
    let pos2 = pos + dir2;
    maybe_push(pos2, dir2, 0, &mut ans, rows, cols);

    // direction 180d
    // skipped since we would be backtracking

    // direction 270d
    let dir4 = dir * ROTATE_ANTI_CLOCKWISE;
    let pos4 = pos + dir4;
    maybe_push(pos4, dir4, 0, &mut ans, rows, cols);

    ans
}

fn a_star(grid: &Vec<Vec<Tile>>, start: &Tile, goal: &Tile) -> i32 {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut neighbor_nodes = HashMap::new();

    let start_node = Node {
        xy: Complex {
            re: start.x as i32,
            im: start.y as i32,
        },
        x: start.x,
        y: start.y,
        direction: Complex { re: 1, im: 0 },
        consecutive: 0,
        g_score: Some(0),
        f_score: h(&grid[start.x][start.y], &goal),
    };

    let mut open_nodes = BinaryHeap::with_capacity(1e6 as usize);
    open_nodes.push(Reverse(start_node));

    // walk across map
    while !open_nodes.is_empty() {
        // retrieve node with the lowest f-score
        let Reverse(curr) = open_nodes.pop().unwrap();

        if curr.x == goal.x && curr.y == goal.y {
            // reached the goal
            return curr.g_score.unwrap();
        }

        // visit neighbors
        // note the difference with the "classic" A-star algorithm,
        // there are 4 parameters instead of only xy-coordinates
        let neighbors = neighbor_nodes
            .entry((curr.xy, curr.direction, curr.consecutive))
            .or_insert_with(|| {
                get_neighbors(curr.xy, curr.direction, curr.consecutive, rows, cols)
            });
        for n in neighbors {
            let tmp_g_score = curr.g_score.unwrap() + d(&grid[n.x][n.y]);
            let g_score_neighbor = n.g_score.unwrap_or(i32::MAX);
            if tmp_g_score < g_score_neighbor {
                // found better path
                n.g_score = Some(tmp_g_score);
                n.f_score = tmp_g_score + h(&grid[n.x][n.y], &goal);
                open_nodes.push(Reverse(*n));
            }
        }
    }

    // no path found from start to goal
    -1
}
