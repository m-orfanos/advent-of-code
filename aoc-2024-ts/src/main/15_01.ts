import { Compass } from "./utils/compass.ts";

export function solve(input: string): number {
  // parse input
  const {
    sx,
    sy,
    moves,
    grid,
  } = parse(input);

  // run simulation
  let [px, py] = [sx, sy];
  for (const [dx, dy] of moves) {
    const [qx, qy] = [px + dx, py + dy];
    const cell = grid[qx][qy];
    if (cell === "#") {
      // hit a wall, do nothing
    } else if (cell === ".") {
      // move to open space
      grid[px][py] = ".";
      grid[qx][qy] = "@";
      [px, py] = [qx, qy];
    } else if (cell === "O") {
      // try move boxes along direction
      let cnt = 1;
      while (true) {
        const [rx, ry] = [qx + cnt * dx, qy + cnt * dy];
        const cell = grid[rx][ry];
        if (cell === "#") {
          // can't move, do nothing
          break;
        } else if (cell === ".") {
          // move all boxes to this point
          // swap the first/last cells since everything
          // in between is the same
          grid[px][py] = ".";
          grid[qx][qy] = "@";
          grid[rx][ry] = "O";
          [px, py] = [qx, qy];
          break;
        }
        cnt += 1;
      }
    }
  }

  // calculate GPS sum
  let sum = 0;
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      if (grid[i][j] === "O") {
        sum += 100 * i + j;
      }
    }
  }

  return sum;
}

function parse(input: string) {
  const grid: string[][] = [];
  const rows = input.split("\n");
  let [sx, sy] = [-1, -1];

  // parse grid
  let i = 0;
  while (true) {
    const row = rows[i].trim();
    if (row.length === 0) {
      i += 1;
      break;
    }
    grid.push([]);

    const cols = row.split("");
    for (let j = 0; j < cols.length; j++) {
      const col = cols[j].trim();
      if (col === "@") {
        [sx, sy] = [i, j];
      }
      grid[i].push(col);
    }
    i += 1;
  }

  // parse moves
  const tmoves: [number, number][][] = [];
  while (i < rows.length) {
    const c = rows[i].trim();
    if (c.length === 0) {
      break;
    }
    tmoves.push([]);

    const cmoves = tmoves[tmoves.length - 1];
    for (const col of rows[i].trim().split("")) {
      switch (col) {
        case "^":
          cmoves.push(Compass.NORTH);
          break;
        case ">":
          cmoves.push(Compass.EAST);
          break;
        case "v":
          cmoves.push(Compass.SOUTH);
          break;
        case "<":
          cmoves.push(Compass.WEST);
          break;
      }
    }

    i += 1;
  }

  const moves = tmoves.flat(1);

  return { sx, sy, moves, grid };
}
