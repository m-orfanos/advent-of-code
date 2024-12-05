import { Direction, Point } from "../utils/compass.ts";
import { toArrayString } from "../utils/parsers.ts";

export function solve(input: string): number {
  const pattern = "XMAS";
  let cnt = 0;
  const grid = toArrayString(input);

  // sliding window approach
  // for each letter (i,j) in the grid
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      // build a word along each ordinal direction
      for (const d of Direction.DIR8) {
        let word = grid[i][j];
        let p = new Point(i, j);

        // append three letters
        for (let k = 0; k < 3; k++) {
          const tmp = new Point(p.x + d.x, p.y + d.y);
          if (
            0 <= tmp.x && tmp.x < grid.length &&
            0 <= tmp.y && tmp.y < grid[i].length
          ) {
            word += grid[tmp.x][tmp.y];
          }
          p = tmp;
        }

        if (word === pattern) {
          cnt += 1;
        }
      }
    }
  }

  return cnt;
}
