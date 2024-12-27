import { parse } from "./24_01.ts";

export function solve(input: string): string {
  // tried brute force, didn't work :/
  //
  // we are essentially trying to simulate a ripple carry adder
  //
  // https://en.wikipedia.org/wiki/Adder_(electronics)#Ripple-carry_adder
  //
  // graphic taken from
  // https://old.reddit.com/r/adventofcode/comments/1hl698z/2024_day_24_solutions/m3l9glk/
  //
  // Case 1
  //   x00──┬─────┐
  //        │    XOR──z00
  //   y00────┬───┘
  //        │ │
  //        │ └───┐
  //        │    AND──mgk
  //        └─────┘
  //
  // Case 2
  //   x01──┬─────┐
  //        │    XOR[rkf]┬────┐
  //   y01────┬───┘      │   XOR──────z01
  //        │ │          │    │
  //   mgk──────┬─────────────┘
  //        │ │ │        │
  //        │ │ │        │
  //        │ │ │       AND[kmj]──┐
  //        │ │ └────────┘        OR──rwp
  //        │ └──────────┐        │
  //        │           AND[nfw]──┘
  //        └────────────┘
  //
  // For every triple... validate it follows the format above.
  // Note: This solution works because only the OUTPUTS are swappable.
  const { circuit } = parse(input);
  const errors = [];
  for (const [a, b, o, s] of circuit) {
    if (
      s[0] == "z" &&
      s != "z45" &&
      o != "XOR"
    ) {
      errors.push(s);
    } else if (
      s[0] != "z" &&
      (a[0] != "x" && a[0] != "y") &&
      (b[0] != "x" && b[0] != "y") &&
      o == "XOR"
    ) {
      errors.push(s);
    } else if (
      (a[0] == "x" || a[0] == "y") &&
      (b[0] == "x" || b[0] == "y") &&
      a[1] != "0" && a[2] != "0" &&
      b[1] != "0" && b[2] != "0"
    ) {
      // case 1B and 2C
      const nextGate = o == "XOR" ? "XOR" : "OR";

      const existsValidGate = circuit
        .filter(([oa, ob, _, os]) => {
          // ignore current
          return oa != a && ob != b && os != s;
        })
        .some(([oa, ob, oo, _]) => {
          return (oa == s || ob == s) && oo === nextGate;
        });
      if (!existsValidGate) {
        errors.push(s);
      }
    }
  }

  errors.sort();

  return errors.join(",");
}
