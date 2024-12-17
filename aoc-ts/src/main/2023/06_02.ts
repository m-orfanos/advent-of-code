import { to1DArrayNumeric } from "../utils/parsers.ts";
import { binarySearch } from "./06_01.ts";

export function solve(input: string): number {
  const rows = input.trim().split("\n");

  const times = to1DArrayNumeric(rows[0].trim().split(":")[1]);
  const distances = to1DArrayNumeric(rows[1].trim().split(":")[1]);

  const t = Number.parseInt(times.join(""), 10);
  const dst = Number.parseInt(distances.join(""), 10);

  const lhs = binarySearch(t, dst, (a, b) => a <= b, (l, _) => l);
  const rhs = binarySearch(t, dst, (a, b) => a > b, (_, r) => r);

  return rhs - lhs + 1;
}
