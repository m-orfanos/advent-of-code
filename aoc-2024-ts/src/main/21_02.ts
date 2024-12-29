import { zip } from "./utils/arrays.ts";
import { eq, sub } from "./utils/compass.ts";
import { dijkstra } from "./utils/grid.ts";
import { to1DArrayString } from "./utils/parsers.ts";

export function solve(input: string): number {
  // "borrowed" the recursive function from the following github repository
  // https://github.com/grant-dot-dev/advent_of_code_2024/blob/main/Day21_Python/solution.py
  function go(code: string, limit: number, depth: number): number {
    if (cache[code]?.[limit]?.[depth]) {
      return cache[code][limit][depth];
    }

    let current = depth == 0 ? "A" : "a";
    let mlen = 0;

    for (const target of code) {
      if (depth == limit) {
        mlen += moves[current][target][0].length;
      } else {
        const nums = [];
        for (const rem of moves[current][target]) {
          nums.push(go(rem, limit, depth + 1));
        }
        mlen += nums.reduce((a, b) => a < b ? a : b);
      }
      current = target;
    }

    if (!cache[code]) {
      cache[code] = {};
    }
    if (!cache[code][limit]) {
      cache[code][limit] = {};
    }
    cache[code][limit][depth] = mlen;

    return mlen;
  }

  const codes = to1DArrayString(input).map((r) => r.trim());
  const moves = {
    ...findKeyPadPaths(),
    ...findDirectionalPadPaths(),
  };

  const cache: { [key: string]: { [key: number]: { [key: number]: number } } } = {};

  let sum = 0;
  for (const code of codes) {
    const v = Number.parseInt(code, 10);
    const t = go(code, 25, 0);
    sum += t * v;
  }

  return sum;
}

function findDirectionalPadPaths() {
  const directionalPad = [
    ["#", "^", "a"],
    ["<", "v", ">"],
  ];

  return findPathsFor(directionalPad);
}

function findKeyPadPaths() {
  const keyPad: string[][] = [
    ["7", "8", "9"],
    ["4", "5", "6"],
    ["1", "2", "3"],
    ["#", "0", "A"],
  ];

  return findPathsFor(keyPad);
}

function findPathsFor(pad: string[][]) {
  const coords: [number, number][] = [];
  for (let i = 0; i < pad.length; i++) {
    for (let j = 0; j < pad[i].length; j++) {
      if (pad[i][j] !== "#") {
        coords.push([i, j]);
      }
    }
  }

  const mpaths: { [key: string]: { [key: string]: string[] } } = {};
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
        sequence.push("a");
        const k1 = pad[p[0]][p[1]];
        const k2 = pad[q[0]][q[1]];
        if (!mpaths[k1]) {
          mpaths[k1] = {};
        }
        if (!mpaths[k1][k2]) {
          mpaths[k1][k2] = [];
        }
        mpaths[k1][k2].push(sequence.join(""));
      }
    }
  }

  return mpaths;
}
