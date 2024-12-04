import { convertTo2DArrayString } from "../utils/convertTo2DArray.ts";

export function solve(input: string): number {
  const coords = [
    [[-1, 0], [-2, 0], [-3, 0]], //    N
    [[0, 1], [0, 2], [0, 3]], //       E
    [[1, 0], [2, 0], [3, 0]], //       S
    [[0, -1], [0, -2], [0, -3]], //    W

    [[-1, 1], [-2, 2], [-3, 3]], //    NE
    [[1, 1], [2, 2], [3, 3]], //       SE
    [[1, -1], [2, -2], [3, -3]], //    SW
    [[-1, -1], [-2, -2], [-3, -3]], // NW
  ];

  const pattern = "XMAS";
  let cnt = 0;
  const grid = convertTo2DArrayString(input);
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      for (const [[x1, y1], [x2, y2], [x3, y3]] of coords) {
        if (
          // second letter
          0 <= i + x1 && i + x1 < grid.length &&
          0 <= j + y1 && j + y1 < grid[i].length &&
          // third letter
          0 <= i + x2 && i + x2 < grid.length &&
          0 <= j + y2 && j + y2 < grid[i].length &&
          // fourth letter
          0 <= i + x3 && i + x3 < grid.length &&
          0 <= j + y3 && j + y3 < grid[i].length
        ) {
          const word = grid[i][j] +
            grid[i + x1][j + y1] +
            grid[i + x2][j + y2] +
            grid[i + x3][j + y3];
          if (word === pattern) {
            cnt += 1;
          }
        }
      }
    }
  }

  return cnt;
}
