import { add, Compass, h2, isBounded, mul } from "../utils/compass.ts";
import { to2DArrayString } from "../utils/parsers.ts";

export function solve(input: string): number {
  // parse input
  const grid = to2DArrayString(input);
  const { source, target } = findStartAndEnd(grid);

  // walk
  const { best } = dijkstra(source, target, grid);

  return best;
}

export function findStartAndEnd(grid: string[][]) {
  let source: [number, number];
  let target: [number, number];
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      if (grid[i][j] === "S") {
        source = [i, j];
        grid[i][j] = ">";
      } else if (grid[i][j] === "E") {
        target = [i, j];
      }
    }
  }

  return {
    source: source!,
    target: target!,
  };
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

/**
 * Insanely scuffed "priority queue" :D
 */
export class PriorityQueue<T> {
  data: [number, T][];
  isSorted: boolean;

  constructor() {
    this.data = [];
    this.isSorted = false;
  }

  isEmpty(): boolean {
    return this.data.length === 0;
  }

  push(priority: number, item: T): void {
    this.data.push([priority, item]);
    this.isSorted = false;
  }

  pop(): T {
    if (!this.isSorted) {
      this.data.sort((a, b) => b[0] - a[0]);
      this.isSorted = true;
    }
    return this.data.pop()![1];
  }
}
