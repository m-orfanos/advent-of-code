import { to2DArrayString } from "../utils/parsers.ts";
import { findGuard } from "./06_01.ts";

export function solve(input: string): number {
  const grid = to2DArrayString(input);

  const [xs, ys] = findGuard(grid);
  const obstacles = findPotentialObstacles(grid);

  let cnt = 0;
  for (const [i, j] of obstacles) {
    // starting position
    let [x, y] = [xs, ys];

    // walking direction
    let [dx, dy] = [-1, 0];

    // add obstacle
    const tmp = grid[i][j];
    grid[i][j] = "#";

    // keep track when visiting coordinates with direction
    const visited = new Set<string>();
    while (
      0 <= x + dx && x + dx < grid.length &&
      0 <= y + dy && y + dy < grid[x].length
    ) {
      if (visited.has(`${x}|${y}|${dx}|${dy}`)) {
        cnt += 1;
        break;
      }

      if (grid[x + dx][y + dy] === "#") {
        // ran into an obstacle
        // turn right and try another path
        [dx, dy] = [dy, -dx];
      } else {
        visited.add(`${x}|${y}|${dx}|${dy}`);
        [x, y] = [x + dx, y + dy];
      }
    }

    // remove obstacle
    grid[i][j] = tmp;
  }

  return cnt;
}

function findPotentialObstacles(grid: string[][]): number[][] {
  // starting position
  let [x, y] = findGuard(grid);

  // walking direction
  let [dx, dy] = [-1, 0];

  const visited = new Set<string>();

  while (
    0 <= x + dx && x + dx < grid.length &&
    0 <= y + dy && y + dy < grid[x].length
  ) {
    if (grid[x + dx][y + dy] === "#") {
      // ran into an obstacle
      // turn right and try another path
      [dx, dy] = [dy, -1 * dx];
    } else {
      visited.add(`${x + dx}|${y + dy}`);
      [x, y] = [x + dx, y + dy];
    }
  }

  // create possible locations
  const arr: number[][] = [];
  for (const v of visited) {
    arr.push(v.split("|").map((n) => Number.parseInt(n, 10)));
  }

  return arr;
}
