import { isSafeReport } from "./02_01.ts";
import { to2DArrayNumeric } from "../utils/parsers.ts";

export function solve(input: string): number {
  let cnt = 0;
  const grid: number[][] = to2DArrayNumeric(input);
  for (let i = 0; i < grid.length; i++) {
    const safe = isSafeReport(grid[i]);
    if (safe) {
      cnt += 1;
      continue;
    }

    // TODO: optimize search space
    for (let j = 0; j < grid[i].length; j++) {
      const cpy = [...grid[i]];
      cpy.splice(j, 1);
      const safe = isSafeReport(cpy);
      if (safe) {
        cnt += 1;
        break;
      }
    }
  }

  return cnt;
}
