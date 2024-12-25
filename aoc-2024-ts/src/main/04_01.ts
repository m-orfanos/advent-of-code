import { Compass } from "./utils/compass.ts";
import { isBounded } from "./utils/grid.ts";
import { to2DArrayString } from "./utils/parsers.ts";

export function solve(input: string): number {
  const grid = to2DArrayString(input);

  const pattern = "XMAS";
  let cnt = 0;

  // sliding window approach
  // for each letter (i,j) in the grid
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      // build a word along each ordinal direction
      for (const d of Compass.DIR8) {
        let word = grid[i][j];
        let [px, py] = [i, j];

        // append three letters
        for (let k = 0; k < 3; k++) {
          const [nx, ny] = [px + d[0], py + d[1]];
          if (isBounded([nx, ny], grid)) {
            word += grid[nx][ny];
          }
          [px, py] = [nx, ny];
        }

        if (word === pattern) {
          cnt += 1;
        }
      }
    }
  }

  return cnt;
}
