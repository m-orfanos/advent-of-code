import { zip } from "./utils/arrays.ts";
import { eq, sub } from "./utils/compass.ts";
import { dijkstra } from "./utils/grid.ts";
import { to1DArrayString } from "./utils/parsers.ts";

export function solve(input: string): number {
  const mkeypad: { [key: string]: string[] } = buildKeyPadMap();
  const mdpad = buildDirectionalPadMap();

  let sum = 0;
  const codes = to1DArrayString(input).map((r) => r.trim());
  for (const code of codes) {
    let min1 = Infinity;
    let min2 = Infinity;
    let min3 = Infinity;
    const moves1: string[] = buildMoves(code, mkeypad);
    for (const move1 of moves1) {
      if (move1.length > min1) {
        continue;
      }
      const moves2 = buildMoves(move1, mdpad);
      for (const move2 of moves2) {
        if (move2.length > min2) {
          continue;
        }
        const moves3 = buildMoves(move2, mdpad);
        for (const move3 of moves3) {
          if (move3.length < min3) {
            min1 = move1.length;
            min2 = move2.length;
            min3 = move3.length;
          }
        }
      }
    }

    sum += Number.parseInt(code, 10) * min3;
  }

  return sum;
}

function buildMoves(code: string, mkeypad: { [key: string]: string[] }) {
  let moves: string[] = [""];
  for (const [a, b] of zip(`A${code}`.split(""), code.split(""))) {
    let next: string[] = [];
    for (const move of moves) {
      for (const sub of mkeypad[`${a}|${b}`]) {
        next.push(move + sub);
      }
    }
    moves = next;
    next = [];
  }
  return moves;
}

function buildDirectionalPadMap() {
  const pad = [
    ["#", "^", "A"],
    ["<", "v", ">"],
  ];

  return buildMap(pad);
}

function buildKeyPadMap() {
  const pad: string[][] = [
    ["7", "8", "9"],
    ["4", "5", "6"],
    ["1", "2", "3"],
    ["#", "0", "A"],
  ];

  return buildMap(pad);
}

function buildMap(pad: string[][]) {
  const coords: [number, number][] = [];
  for (let i = 0; i < pad.length; i++) {
    for (let j = 0; j < pad[i].length; j++) {
      if (pad[i][j] !== "#") {
        coords.push([i, j]);
      }
    }
  }

  const mpad: { [key: string]: string[] } = {};
  for (let i = 0; i < coords.length; i++) {
    for (let j = 0; j < coords.length; j++) {
      const p = coords[i];
      const q = coords[j];
      const paths = dijkstra(p, q, pad);
      for (const path of paths) {
        const sequence = [];
        for (const [u, v] of zip(path, path.slice(1))) {
          const d = sub(v, u);
          if (eq(d, [0, 0])) {
            // do nothing
          } else if (eq(d, [-1, 0])) {
            sequence.push("^");
          } else if (eq(d, [0, 1])) {
            sequence.push(">");
          } else if (eq(d, [1, 0])) {
            sequence.push("v");
          } else if (eq(d, [0, -1])) {
            sequence.push("<");
          } else {
            // sanity check
            throw Error("How did you get here...?");
          }
        }
        sequence.push("A");
        if (!mpad[`${pad[p[0]][p[1]]}|${pad[q[0]][q[1]]}`]) {
          mpad[`${pad[p[0]][p[1]]}|${pad[q[0]][q[1]]}`] = [];
        }
        mpad[`${pad[p[0]][p[1]]}|${pad[q[0]][q[1]]}`].push(sequence.join(""));
      }
    }
  }

  return mpad;
}
