import { Direction, Point } from "../utils/compass.ts";
import { convertTo2DArrayString } from "../utils/convertTo2DArray.ts";

export function solve(input: string): number {
  const pattern1 = "MAS";
  const pattern2 = "SAM";
  let cnt = 0;
  const grid = convertTo2DArrayString(input);

  // sliding window approach
  // for each letter (i,j) in the grid
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      if (grid[i][j] !== "A") {
        continue;
      }
      const p = new Point(i, j);

      // build two words along the diagonal
      const coords = [
        [Direction.NORTH_WEST, Direction.SOUTH_EAST],
        [Direction.NORTH_EAST, Direction.SOUTH_WEST],
      ];

      let matches = 0;
      for (const [d1, d2] of coords) {
        const p1 = new Point(p.x + d1.x, p.y + d1.y);
        const p2 = new Point(p.x + d2.x, p.y + d2.y);
        if (
          0 <= p1.x && p1.x < grid.length && 0 <= p1.y && p1.y < grid[i].length &&
          0 <= p2.x && p2.x < grid.length && 0 <= p2.y && p2.y < grid[i].length
        ) {
          const word = grid[p1.x][p1.y] + grid[i][j] + grid[p2.x][p2.y];
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
