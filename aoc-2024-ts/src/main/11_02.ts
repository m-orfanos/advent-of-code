import { to1DArrayNumeric } from "./utils/parsers.ts";

export function solve(input: string): number {
  function count(n: number, b: number) {
    // dynamic programming approach

    // intermediate results are cached/memoized with every recursive call
    if (!cache[n]) {
      cache[n] = {};
    }
    if (cache[n][b]) {
      return cache[n][b];
    }

    // base case, blink/b === 1
    // three distinct cases with every blink
    if (b === 1) {
      if (n === 0) {
        // 0 -> 1
        return 1;
      } else if ((n + "").length % 2 === 0) {
        // n -> lhs(n), rhs(n)
        return 2;
      } else {
        // n -> n * 2024
        return 1;
      }
    }

    // inductive/recursive step
    if (n === 0) {
      cache[n][b] = count(1, b - 1);
    } else if ((n + "").length % 2 === 0) {
      const mid = Math.floor(((n + "").length) / 2);
      const lhs = count(Number.parseInt((n + "").substring(0, mid), 10), b - 1);
      const rhs = count(Number.parseInt((n + "").substring(mid), 10), b - 1);
      cache[n][b] = lhs + rhs;
    } else {
      cache[n][b] = count(n * 2024, b - 1);
    }
    return cache[n][b];
  }

  const cache: { [key: number]: { [key: number]: number } } = {};

  const stones = to1DArrayNumeric(input);
  return stones
    .map((s) => count(s, 75))
    .reduce((acc, curr) => acc + curr, 0);
}
