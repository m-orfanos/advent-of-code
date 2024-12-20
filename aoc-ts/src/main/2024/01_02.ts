import { to2DArrayNumeric } from "../utils/parsers.ts";

export function solve(input: string): number {
  const grid = to2DArrayNumeric(input, "   ");

  const lhs = grid.map((r) => r[0]);
  const rhs = grid.map((r) => r[1]);

  const counter = rhs.reduce((acc, curr) => ({
    ...acc,
    [curr]: 1 + (acc[curr] || 0),
  }), {} as Record<number, number>);

  return lhs.reduce((acc, curr) => acc + curr * (counter[curr] || 0), 0);
}
