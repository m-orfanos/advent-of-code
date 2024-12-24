import { to1DArrayNumeric } from "./utils/parsers.ts";

export function solve(input: string): number {
  let stones = to1DArrayNumeric(input);
  for (let i = 0; i < 25; i++) {
    const tmp = [];
    for (let j = 0; j < stones.length; j++) {
      if (stones[j] === 0) {
        tmp.push(1);
      } else if ((stones[j] + "").length % 2 === 0) {
        const digits = stones[j] + "";
        const mid = Math.floor((digits.length) / 2);
        tmp.push(Number.parseInt(digits.substring(0, mid), 10));
        tmp.push(Number.parseInt(digits.substring(mid), 10));
      } else {
        tmp.push(stones[j] * 2024);
      }
    }
    stones = tmp;
  }

  return stones.length;
}
