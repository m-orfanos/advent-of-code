import { sum, zip } from "../utils/arrays.ts";
import { to2DArrayNumeric } from "../utils/parsers.ts";

export function solve(input: string): number {
  const grid = to2DArrayNumeric(input, "   ");

  const lhs = grid.map((r) => r[0]);
  const rhs = grid.map((r) => r[1]);

  lhs.sort();
  rhs.sort();

  return sum(zip(lhs, rhs).map(([l, r]) => Math.abs(l - r)));
}
