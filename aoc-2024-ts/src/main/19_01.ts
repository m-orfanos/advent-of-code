import { to1DArrayString } from "./utils/parsers.ts";

export function solve(input: string): number {
  // parse
  const tmp = to1DArrayString(input);
  const patterns = tmp[0].split(",").map((s) => s.trim());
  const designs = tmp.splice(2).map((s) => s.trim());

  // modified dfs
  let cnt = 0;
  for (const design of designs) {
    const fpatterns = patterns.filter((p) => design.includes(p));

    const stk: [string[], string[], number][] = [];
    stk.push([[], fpatterns, 0]);
    while (stk.length > 0) {
      const [segments, fpats, start] = stk.pop()!;

      if (segments.join("") == design) {
        cnt += 1;
        break;
      }

      for (const pattern of fpats) {
        let match = true;
        for (let i = 0; i < pattern.length; i++) {
          match &&= design[start + i] == pattern[i];
        }
        if (match) {
          const nfpats = fpats.filter((p) => design.slice(start + pattern.length).includes(p));
          stk.push([[...segments, pattern], nfpats, start + pattern.length]);
        }
      }
    }
  }

  return cnt;
}
