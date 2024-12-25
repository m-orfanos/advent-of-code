import { add, Compass, mul } from "./utils/compass.ts";
import { h2, isBounded } from "./utils/grid.ts";
import { find, to2DArrayString } from "./utils/parsers.ts";
import { PriorityQueue } from "./utils/queue.ts";

export function solve(input: string): number {
  // parse input
  const grid = to2DArrayString(input);
  const source = find("S", grid)!;
  const target = find("E", grid)!;

  // walk
  const { best } = traverse(source, target, grid);

  return best;
}

export function traverse(
  source: [number, number],
  target: [number, number],
  grid: string[][],
) {
  // slightly modified dijkstra
  // need to take into account facing direction

  // init distances
  const dist: { [key: string]: number } = {};
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      for (const d of Compass.DIR4) {
        dist[h2([i, j], d)] = Infinity;
      }
    }
  }
  dist[h2(source, Compass.EAST)] = 0;

  // traverse grid
  const q = new PriorityQueue<[[number, number], [number, number]]>();
  q.push(0, [source, Compass.EAST]);
  while (!q.isEmpty()) {
    const [u, ud] = q.pop();
    const uc = dist[h2(u, ud)];

    // step or rotate
    const nodes: [[number, number], [number, number], number][] = [
      [add(u, ud), ud, uc + 1], //           step forwards
      [u, mul(ud, [0, 1]), uc + 1000], //    rotate counter-clockwise
      [u, mul(ud, [0, -1]), uc + 1000], //   rotate clockwise
      //[u, mul(ud, [-1, 0]), uc + 2000], // ignored, no point facing backwards
    ];

    for (const [v, vd, vc] of nodes) {
      const [vx, vy] = v;
      if (isBounded(v, grid) && grid[vx][vy] !== "#" && vc < dist[h2(v, vd)]) {
        dist[h2(v, vd)] = vc;
        q.push(vc, [v, vd]);
      }
    }
  }

  // find lowest score
  let best = Infinity;
  for (const d of Compass.DIR4) {
    best = Math.min(best, dist[h2(target, d)]);
  }

  return {
    best,
    dist,
  };
}
