import { add, Compass, h2, isBounded, mul } from "../utils/compass.ts";
import { find, to2DArrayString } from "../utils/parsers.ts";
import { PriorityQueue } from "../utils/queue.ts";

export function solve(input: string): number {
  // parse input
  const grid = to2DArrayString(input);
  const source = find("S", grid)!;
  const target = find("E", grid)!;

  // walk
  const { best } = dijkstra(source, target, grid);

  return best;
}

export function dijkstra(
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
  const q = new PriorityQueue<[[number, number], [number, number], number]>();
  q.push(0, [source, Compass.EAST, 0]);
  while (!q.isEmpty()) {
    const [u, du, cost] = q.pop();

    // step or rotate
    const nodes: [[number, number], [number, number], number][] = [
      [add(u, du), du, cost + 1], //           step forwards
      [u, mul(du, [0, 1]), cost + 1000], //    rotate counter-clockwise
      [u, mul(du, [0, -1]), cost + 1000], //   rotate clockwise
      //[u, mul(du, [-1, 0]), cost + 2000], // ignored, no point facing backwards
    ];

    for (const [v, dv, vc] of nodes) {
      const [vx, vy] = v;
      if (isBounded(v, grid) && grid[vx][vy] !== "#" && vc < dist[h2(v, dv)]) {
        dist[h2(v, dv)] = vc;
        q.push(vc, [v, dv, vc]);
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
