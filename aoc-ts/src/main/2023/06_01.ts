import { zip } from "../utils/arrays.ts";
import { toArrayNumeric } from "../utils/parsers.ts";

export function solve(input: string): number {
  const rows = input.trim().split("\n");

  const times = toArrayNumeric(rows[0].trim().split(":")[1]);
  const distances = toArrayNumeric(rows[1].trim().split(":")[1]);
  const races = zip(times, distances);

  let cnt = 1;
  for (const [t, dst] of races) {
    const lhs = binarySearch(t, dst, (a, b) => a <= b, (l, _) => l);
    const rhs = binarySearch(t, dst, (a, b) => a > b, (_, r) => r);
    cnt *= rhs - lhs + 1;
  }

  return cnt;
}

function binarySearch(
  t: number,
  dst: number,
  cmp: (a: number, b: number) => boolean,
  sel: (a: number, b: number) => number,
) {
  let lhs = 1;
  let rhs = t - 1;
  while (lhs <= rhs) {
    const mid = lhs + Math.floor((rhs - lhs) / 2);
    const travel = (t - mid) * mid;
    if (cmp(travel, dst)) {
      lhs = mid + 1;
    } else {
      rhs = mid - 1;
    }
  }
  return sel(lhs, rhs);
}
