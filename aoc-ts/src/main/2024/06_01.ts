import { new2DArray } from "../utils/arrays.ts";
import { to2DArrayString } from "../utils/parsers.ts";

export function solve(input: string): number {
  const grid = to2DArrayString(input);
  const visited = new2DArray(grid.length, grid[0].length, () => 0);

  // starting position
  let [x, y] = findGuard(grid);
  visited[x][y] = 1;

  // walking direction
  let [dx, dy] = [-1, 0];

  while (
    0 <= x + dx && x + dx < grid.length &&
    0 <= y + dy && y + dy < grid[x].length
  ) {
    if (grid[x + dx][y + dy] === "#") {
      // ran into an obstacle
      // turn right and try another path
      [dx, dy] = [dy, -dx];
    } else {
      visited[x + dx][y + dy] = 1;
      [x, y] = [x + dx, y + dy];
    }
  }

  return visited
    .reduce((acc, curr) => acc.concat(curr), [])
    .reduce((acc, curr) => acc + curr, 0);
}

export function findGuard(grid: string[][]) {
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
