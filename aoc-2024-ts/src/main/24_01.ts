import { to1DArrayString } from "./utils/parsers.ts";

export function solve(input: string): number {
  let { circuit, wires } = parse(input);

  const mops: { [key: string]: (a: number, b: number) => number } = {
    "AND": (a, b) => a && b,
    "OR": (a, b) => a || b,
    "XOR": (a, b) => a ^ b,
  };

  while (circuit.length > 0) {
    const [lhs, rhs, op, out] = circuit.pop()!;
    if (wires[lhs] != null && wires[rhs] != null) {
      wires[out] = mops[op](wires[lhs], wires[rhs]);
    } else {
      circuit = [[lhs, rhs, op, out], ...circuit];
    }
  }

  return toNumber(wires, "z");
}

export function parse(input: string) {
  const rows = to1DArrayString(input);

  const wires: { [key: string]: number } = {};
  let i = 0;
  while (true) {
    const row = rows[i];
    if (row.trim().length == 0) {
      break;
    }

    const [k, v] = row.split(":").map((r) => r.trim());
    wires[k] = v == "0" ? 0 : 1;
    i += 1;
  }

  const circuit: [string, string, string, string][] = [];
  i += 1;
  while (i < rows.length) {
    const row = rows[i];
    const [ins, out] = row.split("->").map((r) => r.trim());
    let lhs, rhs, op;
    if (ins.includes("AND")) {
      [lhs, rhs] = ins.split(" AND ");
      op = "AND";
    } else if (ins.includes("XOR")) {
      [lhs, rhs] = ins.split(" XOR ");
      op = "XOR";
    } else if (ins.includes("OR")) {
      [lhs, rhs] = ins.split(" OR ");
      op = "OR";
    }
    circuit.push([lhs!, rhs!, op!, out]);
    i += 1;
  }

  return { circuit, wires };
}

function toNumber(wires: { [key: string]: number }, n: string) {
  const bits = Object.keys(wires).filter((w) => w[0] == n);
  bits.sort();
  let bin = "";
  for (const x of bits) {
    bin = wires[x] + bin;
  }
  return Number.parseInt(bin, 2);
}
