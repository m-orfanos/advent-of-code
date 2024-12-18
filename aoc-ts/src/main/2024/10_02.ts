import { Direction } from "../utils/compass.ts";
import { to2DArrayNumeric } from "../utils/parsers.ts";

export function solve(input: string): number {
  const grid = to2DArrayNumeric(input, "");

  // keep track of path taken from start -> end
  const trailheads = new Map<string, number>();
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      if (grid[i][j] !== 0) {
        continue;
      }

      const stk = [[[i, j, 0]]];
      while (stk.length > 0) {
        const path = stk.pop()!;
        const [x, y, n] = path[path.length - 1];
        for (const d of Direction.DIR4) {
          if (
            !(0 <= x + d.x && x + d.x < grid.length &&
              0 <= y + d.y && y + d.y < grid[x + d.x].length)
          ) {
            continue;
          }

          if (n + 1 === 9 && grid[x + d.x][y + d.y] === 9) {
            const cnt = trailheads.get(`(${i},${j})`) || 0;
            trailheads.set(`(${i},${j})`, cnt + 1);
          } else if (grid[x + d.x][y + d.y] === n + 1) {
            const tmp = [...path];
            tmp.push([x + d.x, y + d.y, n + 1]);
            stk.push(tmp);
          }
        }
      }
    }
  }

  return Array.from(trailheads.keys())
    .map((k) => trailheads.get(k)!)
    .reduce((acc, curr) => acc + curr, 0);
}
