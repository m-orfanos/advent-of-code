import { to1DArrayString } from "./utils/parsers.ts";

// TODO: solve using cliques and/or the Bron-Kerbosch algorithm (??)
export function solve(input: string): string {
  const conns: [string, string][] = to1DArrayString(input)
    .map((s) => s.split("-"))
    .map((a) => [a[0], a[1]]);

  const nwm: { [key: string]: Set<string> } = {};
  for (const [a, b] of conns) {
    if (!nwm[a]) {
      nwm[a] = new Set();
    }
    nwm[a].add(b);
    if (!nwm[b]) {
      nwm[b] = new Set();
    }
    nwm[b].add(a);
  }

  let max = 0;
  let ns = "";

  const vs = new Set();
  const stk: [string, string[], string][] = conns.map((c) => [c[0], [c[0]], c[1]]);
  while (stk.length > 0) {
    const [key, network, toAdd] = stk.pop()!;

    if (vs.has(key + "|" + toAdd)) {
      continue;
    }
    vs.add(key + "|" + toAdd);

    if (max < network.length) {
      max = network.length;
      ns = key;
    }

    if (network.every((n) => nwm[toAdd].has(n))) {
      nwm[toAdd]
        .values()
        .filter((n) => !network.includes(n))
        .forEach((nxToAdd) => {
          const nxNetwork = [...network, toAdd];
          nxNetwork.sort();
          stk.push([nxNetwork.join(","), nxNetwork, nxToAdd]);
        });
    }
  }

  return ns;
}
