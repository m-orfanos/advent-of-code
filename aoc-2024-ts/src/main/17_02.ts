import { run } from "./17_01.ts";
import { to1DArrayString } from "./utils/parsers.ts";

export function solve(input: string): number {
  // parse input
  const tmp = to1DArrayString(input);
  // const ra = Number.parseInt(tmp[0].split(":")[1].trim(), 10);
  const rb = Number.parseInt(tmp[1].split(":")[1].trim(), 10);
  const rc = Number.parseInt(tmp[2].split(":")[1].trim(), 10);
  const program = tmp[4].split(":")[1].trim().split(",").map((e) => Number.parseInt(e.trim(), 10));

  // 1) print out first 1e8 numbers
  // try to find a pattern
  // matched about half the program
  //
  // 31013903 2,4,1,2,7,5,4,3,4
  // 50674703 2,4,1,2,7,5,4,3,7
  // 52247567 2,4,1,2,7,5,4,3,7
  // 58539023 2,4,1,2,7,5,4,3,7
  // 91241487 2,4,1,2,7,5,4,3,0
  // 91307023 2,4,1,2,7,5,4,3,0
  // 91392945 2,4,1,2,7,5,4,3,0
  // 91438095 2,4,1,2,7,5,4,3,0
  // 99630095 2,4,1,2,7,5,4,3,0
  // 99695631 2,4,1,2,7,5,4,3,0
  // 99826703 2,4,1,2,7,5,4,3,0
  //
  // 2) writing out instructions
  // try to reverse engineer the output
  //
  // B <- A%8
  // B <- B^1
  // C <- A//2^B
  // B <- B ^ C
  // A <- A//2^3 <----------- !!! THIS INSTRUCTION IS KEY !!!
  // B <- B^7
  // PRINT B%8
  // IF A!=0 GOTO 0 ELSE END
  //
  // because of the marked instruction above we know `ra` will be
  // between 8**(16-1) and 8**16 (where 16 is the length of the program)
  // these are too many to check
  // LOW   35184372088832 (>3e13)
  // HIGH 281474976710656 (>2e14)
  let low = Infinity;

  const stk: [number, number][] = [];
  stk.push([0, 0]);

  // reconstruct the program starting from the end
  // search for a match (0-7) on the nth digit of the program
  // on the next iteration, multiply the number found in
  // the previous step by 8 and go agane
  while (stk.length > 0) {
    const [a, n] = stk.pop()!;
    const lastNDigits = program.slice(program.length - 1 - n).join(",");

    for (let i = 0; i < 8; i++) {
      const val = 8 * a + i;
      const output = run(val, rb, rc, program);
      if (output == program.join(",")) {
        // found a full match
        low = Math.min(low, val);
      }
      if (output == lastNDigits) {
        stk.push([val, n + 1]);
      }
    }
  }

  return low;
}
