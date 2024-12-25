import { Heap } from "npm:heap-js";
import { add, Compass, eq } from "./compass.ts";

export function h1(a: [number, number]) {
  return a.join("|");
}

export function h2(a: [number, number], b: [number, number]) {
  return [...a, ...b].join("|");
}

export function isBounded<T>(p: [number, number], grid: T[][]): boolean {
  return 0 <= p[0] && p[0] < grid.length &&
    0 <= p[1] && p[1] < grid[p[0]].length;
}

export function manhattanDistance(u: [number, number], v: [number, number]) {
  // https://en.wikipedia.org/wiki/Taxicab_geometry
  return Math.abs(u[0] - v[0]) + Math.abs(u[1] - v[1]);
}

export function dijkstra<T>(
  source: [number, number],
  target: [number, number],
  grid: T[][],
) {
  // https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
  function reconstructPaths() {
    const paths = [];
    const stk = [[target]];
    while (stk.length > 0) {
      const tpath = stk.pop()!;
      const n = tpath[tpath.length - 1];
      if (eq(n, source)) {
        paths.push(tpath);
        continue;
      }
      for (const m of prev[h1(n)]) {
        stk.push([...tpath, m]);
      }
    }

    return paths;
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

  const prev: { [key: string]: [number, number][] } = {};

  // traverse grid
  const q = new Heap<[number, [number, number]]>((a, b) => a[0] - b[0]);
  q.push([0, source]);
  while (!q.isEmpty()) {
    const [_, u] = q.pop()!;

    for (const v of neighbors(u)) {
      const alt = dist[h1(u)] + d(u, v);
      if (alt < dist[h1(v)]) {
        prev[h1(v)] = [u];
        dist[h1(v)] = alt;
        q.push([alt, v]);
      } else if (alt == dist[h1(v)]) {
        if (!prev[h1(v)]) {
          prev[h1(v)] = [];
        }
        prev[h1(v)].push(u);
      }
    }
  }

  return reconstructPaths();
}

export function aStar<T>(
  start: [number, number],
  goal: [number, number],
  grid: T[][],
) {
  // https://en.wikipedia.org/wiki/A*_search_algorithm
  function reconstructPath(current: [number, number]) {
    const path = [current];
    while (from[h1(current)]) {
      current = from[h1(current)];
      path.push(current);
    }
    return path;
  }

  function h(u: [number, number], v: [number, number]): number {
    return manhattanDistance(u, v);
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
  switch (typeof grid[start[0]][start[1]]) {
    case "number":
      obstacle = Infinity;
      break;
    case "string":
      obstacle = "#";
      break;
    default:
      throw Error(`Type not supported`);
  }

  const from: { [key: string]: [number, number] } = {};

  const gScore: { [key: string]: number } = {};
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      gScore[h1([i, j])] = Infinity;
    }
  }
  gScore[h1(start)] = 0;

  const open = new Set();
  open.add(h1(start));

  const q = new Heap<[number, [number, number]]>((a, b) => a[0] - b[0]);
  q.add([h(start, goal), start]);
  while (!q.isEmpty()) {
    const [_, current] = q.pop()!;
    open.delete(h1(current));

    if (eq(current, goal)) {
      return reconstructPath(current);
    }

    for (const neighbor of neighbors(current)) {
      const cost = gScore[h1(current)] + d(current, neighbor);
      if (cost < gScore[h1(neighbor)]) {
        from[h1(neighbor)] = current;
        gScore[h1(neighbor)] = cost;
        if (!open.has(h1(neighbor))) {
          open.add(h1(neighbor));
          q.push([cost + h(current, neighbor), neighbor]);
        }
      }
    }
  }

  return [];
}
