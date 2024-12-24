import { new2DArray } from "./utils/arrays.ts";
import { isBounded, mul } from "./utils/compass.ts";
import { to2DArrayString } from "./utils/parsers.ts";

export function solve(input: string): number {
  const grid = to2DArrayString(input);
  const visited = new2DArray(grid.length, grid[0].length, () => 0);

  // starting position and direction
  const [px, py] = findGuard(grid);
  visited[px][py] = 1;

  const stk = [[[px, py], [-1, 0]]];
  while (stk.length > 0) {
    const [[qx, qy], [dx, dy]] = stk.pop()!;
    const [rx, ry] = [qx + dx, qy + dy];
    if (isBounded([rx, ry], grid)) {
      if (grid[rx][ry] === "#") {
        // ran into an obstacle
        // turn right and try another path
        const dn = mul([dx, dy], [0, -1]);
        stk.push([[qx, qy], dn]);
      } else {
        visited[rx][ry] = 1;
        stk.push([[rx, ry], [dx, dy]]);
      }
    }
  }

  return visited
    .reduce((acc, curr) => acc.concat(curr), [])
    .reduce((acc, curr) => acc + curr, 0);
}

export function findGuard(grid: string[][]): [number, number] {
  let guard: [number, number] | null = null;
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      if (grid[i][j] === "^") {
        guard = [i, j];
        break;
      }
    }
    if (guard) {
      break;
    }
  }
  return guard!;
}
