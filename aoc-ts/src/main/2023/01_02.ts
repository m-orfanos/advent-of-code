import { kmpSearch } from "../utils/kmpSearch.ts";

export function solve(input: string): number {
  const space: [string, number][] = [
    ["0", 0],
    ["1", 1],
    ["2", 2],
    ["3", 3],
    ["4", 4],
    ["5", 5],
    ["6", 6],
    ["7", 7],
    ["8", 8],
    ["9", 9],
    ["zero", 0],
    ["one", 1],
    ["two", 2],
    ["three", 3],
    ["four", 4],
    ["five", 5],
    ["six", 6],
    ["seven", 7],
    ["eight", 8],
    ["nine", 9],
  ];

  let calibration = 0;
  const rows = input.trim().split("\n");
  for (const row of rows) {
    let ilhs = row.length - 1;
    let vlhs = NaN;

    let irhs = 0;
    let vrhs = NaN;

    for (const [needle, value] of space) {
      const matches = kmpSearch(row, needle);
      if (matches.length > 0) {
        const first = matches[0];
        if (first <= ilhs) {
          ilhs = first;
          vlhs = value;
        }

        const last = matches[matches.length - 1];
        if (last >= irhs) {
          irhs = last;
          vrhs = value;
        }
      }
    }

    calibration += vlhs * 10 + vrhs;
  }

  return calibration;
}
