import { mod } from "./utils/maths.ts";
import { to1DArrayNumeric } from "./utils/parsers.ts";

export function solve(input: string): number {
  let sum = 0;
  const n = 2000;
  const secrets = to1DArrayNumeric(input, "\n");
  for (const secret of secrets) {
    let tmp = secret;
    for (let i = 0; i < n; i++) {
      tmp = mod(tmp ^ (tmp * 64), 16777216);
      tmp = mod(tmp ^ (Math.floor(tmp / 32)), 16777216);
      tmp = mod(tmp ^ (tmp * 2048), 16777216);
    }
    sum += tmp;
  }

  return sum;
}
