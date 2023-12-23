use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    io::{self, BufRead},
};

use num_complex::Complex;

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
    let mut grid = HashMap::new();
    let mut nb_rows = 0;
    for (i, input) in io::stdin().lock().lines().enumerate() {
        let line = input.unwrap();
        nb_rows += 1;
        for (j, ch) in line.chars().enumerate() {
            grid.insert((i, j), ch.to_string().parse().unwrap());
        }
    }
    let heat_loss = a_star(&grid, (0, 0), (nb_rows - 1, nb_rows - 1)).unwrap_or(0);
    println!("{}", heat_loss);
}

fn a_star(
    grid: &HashMap<(usize, usize), i32>,
    (x_start, y_start): (usize, usize),
    (x_goal, y_goal): (usize, usize),
) -> Option<i32> {
    let mut neighbor_nodes = HashMap::new();

    let start_node = Node {
        xy: Complex {
            re: x_start as i32,
            im: y_start as i32,
        },
        x: x_start,
        y: y_start,
        direction: Complex { re: 1, im: 0 },
        consecutive: 0,
        g_score: Some(0),
        f_score: h((x_start, y_start), (x_goal, y_goal)),
    };

    let mut open_nodes = BinaryHeap::with_capacity(1e6 as usize);
    open_nodes.push(Reverse(start_node));

    // walk across map
    while !open_nodes.is_empty() {
        // retrieve node with the lowest f-score
        let Reverse(curr) = open_nodes.pop().unwrap();

        if curr.x == x_goal && curr.y == y_goal {
            // reached the goal
            return curr.g_score;
        }

        // visit neighbors
        // note the difference with the "classic" A-star algorithm,
        // there are 4 parameters instead of only xy-coordinates
        let neighbors = neighbor_nodes
            .entry((curr.xy, curr.direction, curr.consecutive))
            .or_insert_with(|| get_neighbors(curr.xy, curr.direction, curr.consecutive, &grid));
        for n in neighbors {
            let tmp_g_score = curr.g_score.unwrap() + d(&grid, (n.x, n.y));
            let g_score_neighbor = n.g_score.unwrap_or(i32::MAX);
            if tmp_g_score < g_score_neighbor {
                // found better path
                n.g_score = Some(tmp_g_score);
                n.f_score = tmp_g_score + h((n.x, n.y), (x_goal, y_goal));
                open_nodes.push(Reverse(*n));
            }
        }
    }

    // no path found from start to goal
    None
}

fn h((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> i32 {
    // manhattan distance
    (x1.abs_diff(x2) + y1.abs_diff(y2)) as i32
}

fn d(grid: &HashMap<(usize, usize), i32>, (x, y): (usize, usize)) -> i32 {
    *grid.get(&(x, y)).unwrap()
}

fn get_neighbors(
    pos: Complex<i32>,
    dir: Complex<i32>,
    consecutive: i32,
    grid: &HashMap<(usize, usize), i32>,
) -> Vec<Node> {
    fn push(
        p: Complex<i32>,
        d: Complex<i32>,
        c: i32,
        v: &mut Vec<Node>,
        grid: &HashMap<(usize, usize), i32>,
    ) {
        let node_grid = grid.get(&(p.re as usize, p.im as usize));
        if node_grid.is_some() {
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
        push(pos1, dir1, consecutive + 1, &mut ans, grid);
    }

    // direction 90d
    let dir2 = dir * ROTATE_CLOCKWISE;
    let pos2 = pos + dir2;
    push(pos2, dir2, 0, &mut ans, grid);

    // direction 180d
    // skipped since we would be backtracking

    // direction 270d
    let dir4 = dir * ROTATE_ANTI_CLOCKWISE;
    let pos4 = pos + dir4;
    push(pos4, dir4, 0, &mut ans, grid);

    ans
}
