import { zip } from "./utils/arrays.ts";
import { to1DArrayString } from "./utils/parsers.ts";

export function solve(input: string): number {
  const rows = to1DArrayString(input);

  let i = 0;
  const locks = [];
  const keys = [];
  while (i < rows.length) {
    let tmp;

    if (rows[i].trim()[0] == "#") {
      tmp = [0, 0, 0, 0, 0];
      locks.push(tmp);
    } else {
      tmp = [-1, -1, -1, -1, -1];
      keys.push(tmp);
    }

    i += 1;

    let cnt = 0;
    while (cnt < 6) {
      const curr = rows[i].trim();
      for (let j = 0; j < curr.length; j++) {
        tmp[j] += curr[j] == "#" ? 1 : 0;
      }
      cnt += 1;
      i += 1;
    }

    i += 1;
  }

  let cnt = 0;
  for (const key of keys) {
    for (const lock of locks) {
      const fits = zip(key, lock).map(([a, b]) => a + b).every((p) => p <= 5);
      if (fits) {
        cnt += 1;
      }
    }
  }

  return cnt;
}
