import { h1 } from "./compass.ts";

export function print<T>(grid: T[][], path: [number, number][]) {
  // used to check bounds/obstacle check in neighbors function
  let obstacle: string | number;
  let empty: string | number;
  switch (typeof grid[0][1]) {
    case "number":
      obstacle = Infinity;
      empty = 0;
      break;
    case "string":
      empty = ".";
      obstacle = "#";
      break;
    default:
      throw Error(`Type not supported`);
  }

  const s = new Set();
  for (let i = 0; i < path.length; i++) {
    s.add(h1(path[i]));
  }

  for (let i = 0; i < grid.length; i++) {
    let r = "";
    for (let j = 0; j < grid[i].length; j++) {
      if (s.has(h1([i, j]))) {
        r += "O";
      } else if (grid[i][j] == obstacle) {
        r += "#";
      } else if (grid[i][j] == empty) {
        r += ".";
      } else {
        r += grid[i][j];
      }
    }
    console.log(r);
  }
  console.log();
}
