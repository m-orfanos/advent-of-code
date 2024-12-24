import { findAntennas } from "./08_01.ts";
import { combs } from "./utils/combinations.ts";
import { isBounded } from "./utils/compass.ts";
import { to2DArrayString } from "./utils/parsers.ts";

export function solve(input: string): number {
  // parse input
  const grid = to2DArrayString(input);
  const antennas = findAntennas(grid);

  // find all possible antinode locations
  const antinodes = new Set();
  for (const frequency in antennas) {
    for (const [[ax, ay], [bx, by]] of combs(antennas[frequency], 2)) {
      antinodes.add(`${ax}|${ay}`);
      antinodes.add(`${bx}|${by}`);

      // direction vector
      const [dx, dy] = [bx - ax, by - ay];

      // from antenna 1, walk away from antenna 2 following direction vector
      const stk = [[ax, ay]];
      while (stk.length > 0) {
        const [px, py] = stk.pop()!;
        const [qx, qy] = [px - dx, py - dy];
        if (isBounded([qx, qy], grid)) {
          antinodes.add(`${qx}|${qy}`);
          stk.push([qx, qy]);
        }
      }

      // from antenna 2, walk away from antenna 1...
      stk.push([bx, by]);
      while (stk.length > 0) {
        const [px, py] = stk.pop()!;
        const [qx, qy] = [px + dx, py + dy];
        if (isBounded([qx, qy], grid)) {
          antinodes.add(`${qx}|${qy}`);
          stk.push([qx, qy]);
        }
      }
    }
  }

  return antinodes.size;
}
