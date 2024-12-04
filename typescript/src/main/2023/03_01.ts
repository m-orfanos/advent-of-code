export function solve(input: string): number {
  const nums = parseNums(input);
  const syms = parseSymbols(input);

  let sum = 0;
  for (const num of nums) {
    const nrow = num[0];
    const ncol = num[1];
    const nlen = num[2];
    for (const sym of syms) {
      const srow = sym[0];
      const scol = sym[1];
      if (
        srow - 1 <= nrow && nrow <= srow + 1 &&
        ncol - 1 <= scol && scol < ncol + nlen + 1
      ) {
        sum += num[3];
        break;
      }
      if (nrow + 1 < srow) {
        break;
      }
    }
  }

  return sum;
}

function parseSymbols(input: string): [number, number, string][] {
  const ignored = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "."];

  const symbols: [number, number, string][] = [];
  const rows = input.trim().split("\n");
  for (let i = 0; i < rows.length; i++) {
    for (let j = 0; j < rows[i].length; j++) {
      const col = rows[i][j];
      if (ignored.indexOf(col) < 0) {
        symbols.push([i, j, col]);
      }
    }
  }

  return symbols;
}

function parseNums(input: string): [number, number, number, number][] {
  const nx: [number, number, number, number][] = [];
  const rows = input.trim().split("\n");
  for (let i = 0; i < rows.length; i++) {
    let m;
    const regex = /(\d+)+/g;
    const row = rows[i];
    while ((m = regex.exec(row.trim())) !== null) {
      nx.push([i, m.index, m[1].length, Number.parseInt(m[1], 10)]);
    }
  }

  return nx;
}
