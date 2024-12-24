import { Heap } from "npm:heap-js";
import { new2DArray } from "./utils/arrays.ts";
import { add, Compass, eq, h1, isBounded, manhattanDistance } from "./utils/compass.ts";
import { to2DArrayNumeric } from "./utils/parsers.ts";

export function solve(input: string): number {
  // parse input
  const obstacles: [number, number][] = to2DArrayNumeric(input, ",").map((o) => [o[0], o[1]]);

  // magic numbers not part of input
  // size of two-dimensional grid (nxn)
  const [nr, nc] = [71, 71];

  // number of bytes to read
  const nb = 1024;

  // create grid
  const grid = new2DArray(nr, nc, () => 0);
  for (let i = 0; i < nb; i++) {
    const [cx, cy] = obstacles[i];
    grid[cx][cy] = Infinity;
  }

  const path = aStar([0, 0], [nr - 1, nc - 1], grid);

  // _print(grid, path, obstacles, nb);

  return path.length - 1;
}

function toObj(arr: [number, number][]): { [key: string]: boolean } {
  const obj: { [key: string]: boolean } = {};
  for (let i = 0; i < arr.length; i++) {
    const [px, py] = arr[i];
    obj[h1([px, py])] = true;
  }
  return obj;
}

export function aStar(
  start: [number, number],
  goal: [number, number],
  grid: number[][],
) {
  // Note: can reuse this function elsewhere but need
  // to update the bounds/obstacle check (i.e. "get neighbors")
  function reconstructPath(current: [number, number]) {
    const totalPath = [current];
    while (Object.keys(cameFrom).some((k) => k == h1(current))) {
      current = cameFrom[h1(current)];
      totalPath.push(current);
    }
    return totalPath;
  }

  function h(u: [number, number], v: [number, number]): number {
    return manhattanDistance(u, v);
  }

  function d(_u: [number, number], _v: [number, number]): number {
    return 1;
  }

  const cameFrom: { [key: string]: [number, number] } = {};

  const gScore: { [key: string]: number } = {};
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      gScore[h1([i, j])] = Infinity;
    }
  }
  gScore[h1(start)] = 0;

  const openSet = new Set();
  openSet.add(h1(start));

  const q = new Heap<[number, [number, number]]>((a, b) => a[0] - b[0]);
  q.add([h(start, goal), start]);
  while (!q.isEmpty()) {
    const [_, current] = q.pop()!;
    openSet.delete(h1(current));

    if (eq(current, goal)) {
      return reconstructPath(current);
    }

    for (const dir of Compass.DIR4) {
      const neighbor = add(current, dir);
      const [nx, ny] = neighbor;
      // check bounds and obstacles
      if (!isBounded(neighbor, grid) || grid[nx][ny] == Infinity) {
        continue;
      }
      const cost = gScore[h1(current)] + d(current, neighbor);
      if (cost < gScore[h1(neighbor)]) {
        cameFrom[h1(neighbor)] = current;
        gScore[h1(neighbor)] = cost;
        if (!openSet.has(h1(neighbor))) {
          openSet.add(h1(neighbor));
          q.push([cost + h(current, neighbor), neighbor]);
        }
      }
    }
  }

  return [];
}

function _print(
  grid: number[][],
  path: [number, number][],
  obstacles: [number, number][],
  nb: number,
) {
  console.log("Path: ", path);
  const mobst: { [key: string]: boolean } = toObj(obstacles.slice(0, nb));
  const mpath: { [key: string]: boolean } = toObj(path);

  for (let j = 0; j < grid.length; j++) {
    let r = "";
    for (let i = 0; i < grid[j].length; i++) {
      if (mpath[h1([i, j])] && mobst[h1([i, j])]) {
        throw Error("How did you get here...?");
      } else if (mpath[h1([i, j])]) {
        r += "O";
      } else if (mobst[h1([i, j])]) {
        r += "#";
      } else {
        r += ".";
      }
    }
    console.log(r);
  }
  console.log();
}
