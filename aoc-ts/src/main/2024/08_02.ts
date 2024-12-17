import { to2DArrayString } from "../utils/parsers.ts";
import { combs } from "../utils/combinations.ts";

import { findAntennas } from "./08_01.ts";

export function solve(input: string): number {
  // parse input
  const grid = to2DArrayString(input);
  const antennas = findAntennas(grid);

  // find all possible antinode locations
  const antinodes = new Set();
  for (const frequency in antennas) {
    for (const [[x1, y1], [x2, y2]] of combs(antennas[frequency], 2)) {
      antinodes.add(`${x1}|${y1}`);
      antinodes.add(`${x2}|${y2}`);

      // direction vector
      const [dx, dy] = [x2 - x1, y2 - y1];

      // from antenna 1, walk away from antenna 2 following direction vector
      const stk = [[x1, y1]];
      while (stk.length > 0) {
        const [x, y] = stk.pop()!;
        if (
          0 <= x - dx && x - dx < grid.length &&
          0 <= y - dy && y - dy < grid[x - dx].length
        ) {
          antinodes.add(`${x - dx}|${y - dy}`);
          stk.push([x - dx, y - dy]);
        }
      }

      // from antenna 2, walk away from antenna 1...
      stk.push([x2, y2]);
      while (stk.length > 0) {
        const [x, y] = stk.pop()!;
        if (
          0 <= x + dx && x + dx < grid.length &&
          0 <= y + dy && y + dy < grid[x + dx].length
        ) {
          antinodes.add(`${x + dx}|${y + dy}`);
          stk.push([x + dx, y + dy]);
        }
      }
    }
  }

  return antinodes.size;
}
