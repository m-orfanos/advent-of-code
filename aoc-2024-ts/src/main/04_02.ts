import { Compass, isBounded } from "./utils/compass.ts";
import { to2DArrayString } from "./utils/parsers.ts";

export function solve(input: string): number {
  const grid = to2DArrayString(input);

  const pattern1 = "MAS";
  const pattern2 = "SAM";
  let cnt = 0;

  // sliding window approach
  // for each letter (i,j) in the grid
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      if (grid[i][j] !== "A") {
        continue;
      }

      // build two words along the diagonal
      const dirs = [
        [Compass.NORTH_WEST, Compass.SOUTH_EAST],
        [Compass.NORTH_EAST, Compass.SOUTH_WEST],
      ];

      let matches = 0;
      for (const [d1, d2] of dirs) {
        const [px, py] = [i + d1[0], j + d1[1]];
        const [qx, qy] = [i + d2[0], j + d2[1]];
        if (isBounded([px, py], grid) && isBounded([qx, qy], grid)) {
          const word = grid[px][py] + grid[i][j] + grid[qx][qy];
          if (word === pattern1 || word === pattern2) {
            matches += 1;
          }
        }
      }

      if (matches >= 2) {
        cnt += 1;
      }
    }
  }

  return cnt;
}
