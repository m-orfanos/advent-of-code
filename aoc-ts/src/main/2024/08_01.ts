import { to2DArrayString } from "../utils/parsers.ts";
import { combs } from "../utils/combinations.ts";

export function solve(input: string): number {
  // parse input
  const grid = to2DArrayString(input);
  const antennas = findAntennas(grid);

  // find all possible antinode locations
  const antinodes = new Set();
  for (const frequency in antennas) {
    for (const [[x1, y1], [x2, y2]] of combs(antennas[frequency], 2)) {
      // direction vector
      const [dx, dy] = [x2 - x1, y2 - y1];

      // from antenna 1, take step away from antenna 2 following direction vector
      if (
        0 <= x1 - dx && x1 - dx < grid.length &&
        0 <= y1 - dy && y1 - dy < grid[x1 - dx].length
      ) {
        antinodes.add(`${x1 - dx}|${y1 - dy}`);
      }

      // from antenna 2, take step away from antenna 1...
      if (
        0 <= x2 + dx && x2 + dx < grid.length &&
        0 <= y2 + dy && y2 + dy < grid[x2 + dx].length
      ) {
        antinodes.add(`${x2 + dx}|${y2 + dy}`);
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
