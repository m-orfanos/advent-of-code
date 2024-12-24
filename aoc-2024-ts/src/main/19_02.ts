import { new1DArray } from "./utils/arrays.ts";
import { to1DArrayString } from "./utils/parsers.ts";

export function solve(input: string): number {
  // parse
  const tmp = to1DArrayString(input);
  const patterns = tmp[0].split(",").map((s) => s.trim());
  const designs = tmp.splice(2).map((s) => s.trim());

  // bottom-up dynamic programming approach
  // "rebuild" the design starting from the end
  let cnt = 0;
  for (const design of designs) {
    const mem = new1DArray(design.length, () => 0);

    const fpatterns = patterns.filter((p) => design.includes(p));
    for (let i = design.length - 1; i >= 0; i--) {
      for (const pattern of fpatterns) {
        if (design.slice(i).indexOf(pattern) == 0) {
          if (i + pattern.length == design.length) {
            mem[i] += 1;
          } else {
            mem[i] += mem[i + pattern.length];
          }
        }
      }
    }

    cnt += mem[0];
  }

  return cnt;
}
