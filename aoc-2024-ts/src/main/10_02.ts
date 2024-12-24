import { Compass, isBounded } from "./utils/compass.ts";
import { to2DArrayNumeric } from "./utils/parsers.ts";

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
        const [px, py, n] = path[path.length - 1];
        for (const [dx, dy] of Compass.DIR4) {
          const [qx, qy] = [px + dx, py + dy];

          if (!isBounded([qx, qy], grid)) {
            continue;
          }

          if (n + 1 === 9 && grid[qx][qy] === 9) {
            const cnt = trailheads.get(`(${i},${j})`) || 0;
            trailheads.set(`(${i},${j})`, cnt + 1);
          } else if (grid[qx][qy] === n + 1) {
            const tmp = [...path];
            tmp.push([qx, qy, n + 1]);
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
