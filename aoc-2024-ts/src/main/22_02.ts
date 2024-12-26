import { mod } from "./utils/maths.ts";
import { to1DArrayNumeric } from "./utils/parsers.ts";

export function solve(input: string): number {
  const n = 2000;
  const m = [];

  const s = new Set<string>();
  const secrets = to1DArrayNumeric(input, "\n");
  for (const secret of secrets) {
    const digits: [number, number][] = [];

    let cur = mod(secret, 10);
    let nxt = secret;
    for (let i = 0; i < n; i++) {
      nxt = mod(nxt ^ (nxt * 64), 16777216);
      nxt = mod(nxt ^ (Math.floor(nxt / 32)), 16777216);
      nxt = mod(nxt ^ (nxt * 2048), 16777216);
      digits.push([mod(nxt, 10), mod(nxt, 10) - cur]);
      cur = mod(nxt, 10);
    }

    const quads: { [key: string]: number } = {};
    for (let i = 0; i < n - 4; i++) {
      const k = digits.slice(i, i + 4).map((a) => a[1]).join("|");
      if (!quads[k]) {
        s.add(k);
        quads[k] = digits[i + 3][0];
      }
    }

    m.push(quads);
  }

  let max = 0;
  for (const seq of s.values()) {
    let sum = 0;
    for (let i = 0; i < secrets.length; i++) {
      sum += m[i][seq] || 0;
    }
    if (sum > max) {
      max = sum;
    }
  }

  return max;
}
