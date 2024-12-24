import { mod } from "./utils/maths.ts";
import { to1DArrayString } from "./utils/parsers.ts";

export function solve(input: string): string {
  // parse input
  const tmp = to1DArrayString(input);
  const ra = Number.parseInt(tmp[0].split(":")[1].trim(), 10);
  const rb = Number.parseInt(tmp[1].split(":")[1].trim(), 10);
  const rc = Number.parseInt(tmp[2].split(":")[1].trim(), 10);
  const program = tmp[4].split(":")[1].trim().split(",").map((e) => Number.parseInt(e.trim(), 10));

  return run(ra, rb, rc, program);
}

export function run(ra: number, rb: number, rc: number, program: number[]) {
  function combo(operand: number) {
    if (0 <= operand && operand <= 3) {
      return operand;
    } else if (operand == 4) {
      return ra;
    } else if (operand == 5) {
      return rb;
    } else if (operand == 6) {
      return rc;
    } else {
      throw Error("How did you get here??");
    }
  }

  let iptr = 0;
  const outputs = [];
  while (iptr < program.length) {
    const opcode = program[iptr];
    const operand = program[iptr + 1];

    if (opcode === 0) {
      // adv division
      ra = Math.floor(ra / Math.pow(2, combo(operand)));
    } else if (opcode === 1) {
      // bxl bitwise XOR
      rb = rb ^ operand;
    } else if (opcode === 2) {
      // bst modulo
      rb = mod(combo(operand), 8);
    } else if (opcode === 3) {
      // jnz jump
      if (ra !== 0) {
        iptr = operand;
        continue;
      }
    } else if (opcode === 4) {
      // bxc bitwise XOR
      rb = rb ^ rc;
    } else if (opcode === 5) {
      // out
      outputs.push(mod(combo(operand), 8));
    } else if (opcode === 6) {
      // bdv division
      rb = Math.floor(ra / Math.pow(2, combo(operand)));
    } else if (opcode === 7) {
      // cdv division
      rc = Math.floor(ra / Math.pow(2, combo(operand)));
    }

    iptr += 2;
  }

  return outputs.join(",");
}
