import { to2DArrayString } from "../utils/parsers.ts";
import { Compass, h1, h2, mul, sub } from "../utils/compass.ts";
import { dijkstra, findStartAndEnd, PriorityQueue } from "./16_01.ts";

export function solve(input: string): number {
  const grid = to2DArrayString(input);
  const { source, target } = findStartAndEnd(grid);

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
    const [u, du, cost] = q.pop();
    visited.add(h1(u));

    const neighbors: [[number, number], [number, number], number][] = [
      [sub(u, du), du, cost - 1],
      [u, mul(du, [0, 1]), cost - 1000],
      [u, mul(du, [0, -1]), cost - 1000],
    ];

    for (const [v, dv, vc] of neighbors) {
      if (vc === dist[h2(v, dv)]) {
        q.push(vc, [v, dv, vc]);
        dist[h2(v, dv)] = Infinity;
      }
    }
  }

  return visited.size;
}
