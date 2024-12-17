import { to1DArrayNumeric } from "../utils/parsers.ts";

export function solve(input: string): number {
  const rows = input.trim().split("\n");
  const cache: number[] = Array<number>(rows.length).fill(1);
  for (let i = 0; i < rows.length; i++) {
    const row = rows[i];
    const card = row.trim().split(":");
    const numbers = card[1].trim().split("|");

    const winners = new Set(to1DArrayNumeric(numbers[0]));
    const player = new Set(to1DArrayNumeric(numbers[1]));

    const overlap = winners.intersection(player);
    for (let j = 0; j < overlap.size; j++) {
      cache[i + j + 1] += cache[i];
    }
  }

  return cache.reduce((acc, curr) => acc + curr, 0);
}
