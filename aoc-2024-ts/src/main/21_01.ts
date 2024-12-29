import { Heap } from "npm:heap-js";

import { new1DArray, zip } from "./utils/arrays.ts";
import { eq, sub } from "./utils/compass.ts";
import { dijkstra } from "./utils/grid.ts";
import { to1DArrayString } from "./utils/parsers.ts";

export function solve(input: string): number {
  const codes = to1DArrayString(input).map((r) => r.trim());
  const mkeypad: { [key: string]: string[] } = buildKeyPadMap();
  const mdpad = buildDirectionalPadMap();

  const cnt = 3 + 1;

  const mlens = codes.reduce((acc, c) => ({
    ...acc,
    [c]: new1DArray(cnt, () => Infinity),
  }), {} as Record<string, number[]>);

  const q = new Heap<[number, string, string, number[]]>((a, b) => b[0] - a[0]);
  codes.forEach((c) => {
    q.push([0, c, c, new1DArray(cnt, () => Infinity)]);
  });

  while (q.length > 0) {
    const [clvl, cmove, ccode, clens] = q.pop()!;
    if (cmove.length > mlens[ccode][clvl]) {
      continue;
    }
    clens[clvl] = cmove.length;

    if (clvl == cnt - 1) {
      mlens[ccode] = clens;
    } else {
      const nmoves = buildMoves(cmove, clvl == 0 ? mkeypad : mdpad);
      for (const nmove of nmoves) {
        q.push([clvl + 1, nmove, ccode, clens]);
      }
    }
  }

  let sum = 0;
  for (const code of codes) {
    sum += Number.parseInt(code, 10) * mlens[code][cnt - 1];
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
