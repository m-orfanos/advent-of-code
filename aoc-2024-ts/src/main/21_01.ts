import { zip } from "./utils/arrays.ts";
import { eq, sub } from "./utils/compass.ts";
import { dijkstra } from "./utils/grid.ts";

export function solve(input: string): number {
  const keypad: string[][] = [
    ["7", "8", "9"],
    ["4", "5", "6"],
    ["1", "2", "3"],
    ["#", "0", "A"],
  ];

  const coords: [number, number][] = [];
  for (let i = 0; i < keypad.length; i++) {
    for (let j = 0; j < keypad[i].length; j++) {
      if (keypad[i][j] !== "#") {
        coords.push([i, j]);
      }
    }
  }

  const sequences01: { [key: string]: string[] } = {};
  for (let i = 0; i < coords.length; i++) {
    for (let j = 0; j < coords.length; j++) {
      const p = coords[i];
      const q = coords[j];
      const paths = dijkstra(p, q, keypad);
      for (const path of paths) {
        const sequence = [];
        for (const [u, v] of zip(path, path.slice(1))) {
          const d = sub(v, u);
          if (eq(d, [0, 0])) {
            // do nothing
          } else if (eq(d, [1, 0])) {
            sequence.push("^");
          } else if (eq(d, [0, -1])) {
            sequence.push(">");
          } else if (eq(d, [-1, 0])) {
            sequence.push("V");
          } else if (eq(d, [0, 1])) {
            sequence.push("<");
          } else {
            // sanity check
            throw Error("How did you get here...?");
          }
        }
        sequence.push("A");
        if (!sequences01[`${keypad[p[0]][p[1]]}|${keypad[q[0]][q[1]]}`]) {
          sequences01[`${keypad[p[0]][p[1]]}|${keypad[q[0]][q[1]]}`] = [];
        }
        sequences01[`${keypad[p[0]][p[1]]}|${keypad[q[0]][q[1]]}`].push(sequence.join(""));
      }
    }
  }

  // level 1 check
  const code = "029A";
  let moves: string[] = [""];
  for (const [a, b] of zip("A029A".split(""), "029A".split(""))) {
    let next: string[] = [];
    for (const move of moves) {
      for (const sub of sequences01[`${a}|${b}`]) {
        next.push(move + sub);
      }
    }
    moves = next;
    next = [];
  }

  for (const move of moves) {
    console.log(code, move);
  }

  return -1;
}
