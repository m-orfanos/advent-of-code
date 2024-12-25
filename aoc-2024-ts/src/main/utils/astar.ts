import { Heap } from "npm:heap-js";
import { add, Compass, eq, h1, isBounded, manhattanDistance } from "./compass.ts";

export function aStar<T>(
  start: [number, number],
  goal: [number, number],
  grid: T[][],
) {
  function reconstructPath(current: [number, number]) {
    const path = [current];
    while (Object.keys(from).some((k) => k == h1(current))) {
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
