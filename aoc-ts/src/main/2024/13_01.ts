import { to1DArrayString } from "../utils/parsers.ts";

export function solve(input: string): number {
  const rows = to1DArrayString(input);

  let sum = 0;
  for (let i = 0; i < rows.length; i += 4) {
    const [ax, ay] = rows[i]
      .split(":")[1].trim()
      .split(",")
      .map((x) => x.trim().split("+")[1])
      .map((x) => Number.parseInt(x, 10));

    const [bx, by] = rows[i + 1]
      .split(":")[1].trim()
      .split(",")
      .map((x) => x.trim().split("+")[1])
      .map((x) => Number.parseInt(x, 10));

    const [px, py] = rows[i + 2]
      .split(":")[1].trim()
      .split(",")
      .map((x) => x.trim().split("=")[1])
      .map((x) => Number.parseInt(x, 10));

    const amax = Math.min(Math.floor(px / ax), Math.floor(py / ay));
    // const bmax = Math.min(Math.floor(px / bx), Math.floor(py / by));

    let min = Infinity;

    for (let j = 0; j <= amax; j++) {
      const k = Math.floor((px - j * ax) / bx);
      if (j * ax + k * bx == px && j * ay + k * by == py) {
        min = Math.min(min, j * 3 + k * 1);
      }
    }

    if (min !== Infinity) {
      sum += min;
    }
  }

  return sum;
}
