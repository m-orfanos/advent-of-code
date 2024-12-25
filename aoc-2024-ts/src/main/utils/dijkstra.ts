import { Heap } from "npm:heap-js";
import { add, Compass, eq, h1, isBounded } from "./compass.ts";

export function dijkstra<T>(
  source: [number, number],
  target: [number, number],
  grid: T[][],
) {
  // Note: there could be many "best" paths, but here
  // we simply return the first one discovered
  function reconstructPath(current: [number, number]) {
    const path = [current];
    while (prev[h1(current)]) {
      current = prev[h1(current)];
      path.push(current);
    }
    return path;
  }

  function d(_u: [number, number], _v: [number, number]): number {
    return 1;
  }

  function neighbors(n: [number, number]) {
    const arr = [];
    for (const dir of Compass.DIR4) {
      const neighbor = add(n, dir);
      const [nx, ny] = neighbor;
      // check bounds and obstacles
      if (!isBounded(neighbor, grid) || grid[nx][ny] == obstacle) {
        continue;
      }
      arr.push(neighbor);
    }
    return arr;
  }

  // used to check bounds/obstacle check in neighbors function
  let obstacle: string | number;
  switch (typeof grid[source[0]][source[1]]) {
    case "number":
      obstacle = Infinity;
      break;
    case "string":
      obstacle = "#";
      break;
    default:
      throw Error(`Type not supported`);
  }

  // init distances
  const dist: { [key: string]: number } = {};
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      dist[h1([i, j])] = Infinity;
    }
  }
  dist[h1(source)] = 0;

  const prev: { [key: string]: [number, number] } = {};

  // traverse grid
  const q = new Heap<[number, [number, number]]>((a, b) => a[0] - b[0]);
  q.push([0, source]);
  while (!q.isEmpty()) {
    const [_, u] = q.pop()!;

    if (eq(u, target)) {
      return reconstructPath(target);
    }

    for (const v of neighbors(u)) {
      const alt = dist[h1(u)] + d(u, v);
      if (alt < dist[h1(v)]) {
        prev[h1(v)] = u;
        dist[h1(v)] = alt;
        q.push([alt, v]);
      }
    }
  }

  return [];
}
