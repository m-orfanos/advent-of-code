import { new2DArray } from "../utils/arrays.ts";
import { to2DArrayNumeric } from "../utils/parsers.ts";
import { aStar } from "./18_01.ts";

export function solve(input: string): [number, number] {
  // parse input
  const obstacles: [number, number][] = to2DArrayNumeric(input, ",").map((o) => [o[0], o[1]]);

  // magic numbers not part of input
  // size of two-dimensional grid (nxn)
  const [nr, nc] = [71, 71];

  // binary search
  let lhs = 0;
  let rhs = obstacles.length;
  while (lhs < rhs) {
    // number of bytes to read
    const mid = lhs + Math.floor((rhs - lhs) / 2);

    // create grid
    const grid = new2DArray(nr, nc, () => 0);
    for (let i = 0; i < mid; i++) {
      const [cx, cy] = obstacles[i];
      grid[cx][cy] = Infinity;
    }

    // walk
    const path = aStar([0, 0], [nr - 1, nc - 1], grid);
    if (path.length > 0) {
      lhs = mid + 1;
    } else {
      rhs = mid - 1;
    }
  }

  return obstacles[lhs - 1];
}
