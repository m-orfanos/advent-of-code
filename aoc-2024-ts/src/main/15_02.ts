import { Compass, Vector } from "./utils/compass.ts";

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
    } else if (cell === "[" || cell === "]") {
      const visited = new Set();
      const moveBoxes = [];

      // try move boxes along direction
      let canPush = true;
      const stk = [[qx, qy]];
      while (stk.length > 0) {
        const [ax, ay] = stk.pop()!;

        if (visited.has(`${ax}|${ay}`)) {
          continue;
        }
        visited.add(`${ax}|${ay}`);
        moveBoxes.push([ax, ay]);

        // check the other half
        if (grid[ax][ay] === "[") {
          stk.push([ax, ay + 1]);
        } else if (grid[ax][ay] === "]") {
          stk.push([ax, ay - 1]);
        }

        // check where I'm moving
        const [bx, by] = [ax + dx, ay + dy];
        if (grid[bx][by] === "#") {
          // can't move, do nothing
          canPush = false;
          break;
        } else if (grid[bx][by] === ".") {
          // could move here, but do nothing for now
          // need to check the remaining boxes
        } else if (grid[bx][by] === "[" || grid[bx][by] === "]") {
          // need to check this box
          stk.push([bx, by]);
        }
      }

      if (canPush) {
        // the order of updates is important
        // (e.g. move the furthest box first)
        // sort the updates wrt to the direction being pushed
        // the other axis can be ignored
        if (dx !== 0) {
          moveBoxes.sort(([ax, _ay], [bx, _by]) => dx * (bx - ax));
        } else {
          moveBoxes.sort(([_ax, ay], [_bx, by]) => dy * (by - ay));
        }
        for (const [x, y] of moveBoxes) {
          grid[x + dx][y + dy] = grid[x][y];
          grid[x][y] = ".";
        }
        grid[qx][qy] = "@";
        grid[px][py] = ".";
        [px, py] = [qx, qy];
      }
    }
  }

  // calculate GPS sum
  let sum = 0;
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      if (grid[i][j] === "[") {
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
        [sx, sy] = [i, 2 * j];
        grid[i].push(col);
        grid[i].push(".");
      } else if (col == "O") {
        grid[i].push("[");
        grid[i].push("]");
      } else {
        grid[i].push(col);
        grid[i].push(col);
      }
    }
    i += 1;
  }

  // parse moves
  const tmoves: Vector[][] = [];
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
