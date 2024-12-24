import { Compass, isBounded } from "./utils/compass.ts";
import { to2DArrayNumeric } from "./utils/parsers.ts";

export function solve(input: string): number {
  const grid = to2DArrayNumeric(input, "");

  // keep track of start and end points only
  // the path taken (start -> end) doesn't matter
  const trailheads = new Map<string, Set<string>>();
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      if (grid[i][j] !== 0) {
        continue;
      }

      const stk = [[i, j, 0]];
      while (stk.length > 0) {
        const [px, py, n] = stk.pop()!;
        for (const [dx, dy] of Compass.DIR4) {
          const [qx, qy] = [px + dx, py + dy];

          if (!isBounded([qx, qy], grid)) {
            continue;
          }

          if (n + 1 === 9 && grid[qx][qy] === 9) {
            if (!trailheads.has(`(${i},${j})`)) {
              trailheads.set(`(${i},${j})`, new Set());
            }
            trailheads.get(`(${i},${j})`)?.add(`(${qx},${qy})`);
          } else if (grid[qx][qy] === n + 1) {
            stk.push([qx, qy, n + 1]);
          }
        }
      }
    }
  }

  return Array.from(trailheads.keys())
    .map((k) => trailheads.get(k)!.size)
    .reduce((acc, curr) => acc + curr, 0);
}
