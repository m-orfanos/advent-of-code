import { to1DArrayString } from "./utils/parsers.ts";

export function solve(input: string): number {
  const nm = to1DArrayString(input)
    .map((s) => s.split("-"))
    .reduce((acc, cur) => ({
      ...acc,
      [cur[0]]: [...(acc[cur[0]] || []), cur[1]],
      [cur[1]]: [...(acc[cur[1]] || []), cur[0]],
    }), {} as Record<string, string[]>);

  const pcs = Object.keys(nm);

  let cnt = 0;
  for (let i = 0; i < pcs.length; i++) {
    for (let j = i + 1; j < pcs.length; j++) {
      for (let k = j + 1; k < pcs.length; k++) {
        if ((pcs[i][0] == "t" || pcs[j][0] == "t" || pcs[k][0] == "t")) {
          const c12 = nm[pcs[i]].includes(pcs[j]);
          const c13 = nm[pcs[i]].includes(pcs[k]);
          const c23 = nm[pcs[j]].includes(pcs[k]);
          if (c12 && c13 && c23) {
            cnt += 1;
          }
        }
      }
    }
  }

  return cnt;
}
