import { find, to2DArrayString } from "../utils/parsers.ts";
import { Compass, h1, h2, mul, sub } from "../utils/compass.ts";
import { PriorityQueue } from "../utils/queue.ts";

import { dijkstra } from "./16_01.ts";

export function solve(input: string): number {
  const grid = to2DArrayString(input);
  const source = find("S", grid)!;
  const target = find("E", grid)!;

  // walk
  const { best, dist } = dijkstra(source, target, grid);

  // find all tiles part of the shortest path(s)
  // start from target and traverse backwards
  const q = new PriorityQueue<[[number, number], [number, number], number]>();
  for (const d of Compass.DIR4) {
    const cost = dist[h2(target, d)];
    if (cost == best) {
      q.push(cost, [target, d, cost]);
    }
  }

  const visited = new Set();
  while (!q.isEmpty()) {
    const [u, ud, uc] = q.pop();
    visited.add(h1(u));

    const neighbors: [[number, number], [number, number], number][] = [
      [sub(u, ud), ud, uc - 1],
      [u, mul(ud, [0, 1]), uc - 1000],
      [u, mul(ud, [0, -1]), uc - 1000],
    ];

    for (const [v, vd, vc] of neighbors) {
      if (vc === dist[h2(v, vd)]) {
        q.push(vc, [v, vd, vc]);
        dist[h2(v, vd)] = Infinity;
      }
    }
  }

  return visited.size;
}
