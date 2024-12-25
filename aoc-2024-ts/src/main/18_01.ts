import { new2DArray } from "./utils/arrays.ts";
import { to2DArrayNumeric } from "./utils/parsers.ts";
import { aStar, h1 } from "./utils/grid.ts";

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
