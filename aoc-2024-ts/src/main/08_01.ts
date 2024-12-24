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
      // direction vector
      const [dx, dy] = [bx - ax, by - ay];

      // from antenna 1, take step away from antenna 2 following direction vector
      const [px, py] = [ax - dx, ay - dy];
      if (isBounded([px, py], grid)) {
        antinodes.add(`${px}|${py}`);
      }

      // from antenna 2, take step away from antenna 1...
      const [qx, qy] = [bx + dx, by + dy];
      if (isBounded([qx, qy], grid)) {
        antinodes.add(`${qx}|${qy}`);
      }
    }
  }

  return antinodes.size;
}

export function findAntennas(grid: string[][]): { [key: string]: [number, number][] } {
  const antennas: { [key: string]: [number, number][] } = {};
  const regex = /[a-zA-Z0-9]/g;
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      const ch = grid[i][j];
      if (ch.match(regex)) {
        if (!antennas[ch]) {
          antennas[ch] = [];
        }
        antennas[ch].push([i, j]);
      }
    }
  }
  return antennas;
}
