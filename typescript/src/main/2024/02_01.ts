import { convertTo2DArrayNumeric } from "../utils/convertTo2DArray.ts";

export function solve(input: string): number {
  const grid: number[][] = convertTo2DArrayNumeric(input);
  return grid.reduce((acc, curr) => acc + (isSafeReport(curr) ? 1 : 0), 0);
}

export function isSafeReport(report: number[]): boolean {
  let isIncreasing = true;
  let isDecreasing = true;
  let isGradual = true;

  for (let j = 1; j < report.length; j++) {
    const prev = report[j - 1];
    const curr = report[j];
    const diff = curr - prev;
    isIncreasing &&= diff < 0;
    isDecreasing &&= diff > 0;
    isGradual &&= 1 <= Math.abs(diff) && Math.abs(diff) <= 3;
  }

  return (isIncreasing || isDecreasing) && isGradual;
}
