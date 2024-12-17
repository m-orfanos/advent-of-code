import { to1DArrayNumeric } from "../utils/parsers.ts";

export function solve(input: string): number {
  let points = 0;

  const rows = input.trim().split("\n");
  for (const row of rows) {
    const card = row.trim().split(":");
    const numbers = card[1].trim().split("|");

    const winners = new Set(to1DArrayNumeric(numbers[0]));
    const player = new Set(to1DArrayNumeric(numbers[1]));

    const overlap = winners.intersection(player);
    if (overlap.size > 0) {
      points += Math.pow(2, overlap.size - 1);
    }
  }

  return points;
}
