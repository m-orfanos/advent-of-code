import { parseNums } from "./03_01.ts";

export function solve(input: string): number {
  const nums = parseNums(input);
  const syms = parseSymbols(input);

  let sum = 0;
  for (const sym of syms) {
    const srow = sym[0];
    const scol = sym[1];
    const adjs = [];
    for (const num of nums) {
      const nrow = num[0];
      const ncol = num[1];
      const nlen = num[2];
      if (
        srow - 1 <= nrow && nrow <= srow + 1 &&
        ncol - 1 <= scol && scol < ncol + nlen + 1
      ) {
        adjs.push(num[3]);
      }
    }
    if (adjs.length === 2) {
      sum += adjs[0] * adjs[1];
    }
  }

  return sum;
}

function parseSymbols(input: string): [number, number, string][] {
  const symbols: [number, number, string][] = [];
  const rows = input.trim().split("\n");
  for (let i = 0; i < rows.length; i++) {
    for (let j = 0; j < rows[i].length; j++) {
      const col = rows[i][j];
      if (col === "*") {
        symbols.push([i, j, col]);
      }
    }
  }

  return symbols;
}
