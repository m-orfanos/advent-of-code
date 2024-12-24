import { to1DArrayString } from "./utils/parsers.ts";

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
      .map((x) => Number.parseInt(x, 10) + 10000000000000);

    // Button A: X+92, Y+24
    // Button B: X+13, Y+94
    // Prize: X=8901, Y=8574
    //
    // Create a system of linear equations
    // 94m + 22n = 10000000000000 + 8400
    // 34m + 67n = 10000000000000 + 5400
    //
    // put them in matrix form and solve for B
    // A * B = C
    //
    // calculate the inverse
    // B = inv(A) * C
    //
    // there's an analytical formula for that :))
    // https://en.wikipedia.org/wiki/Invertible_matrix#Inversion_of_2_%C3%97_2_matrices
    //
    // Note: this approach only works because ALL the cases in the input are invertible...

    const m = Math.floor((by * px - bx * py) / ((ax * by) - (bx * ay)));
    const n = Math.floor((-ay * px + ax * py) / ((ax * by) - (bx * ay)));

    if ((m * ax + n * bx == px) && (m * ay + n * by == py)) {
      sum += m * 3 + n * 1;
    }
  }

  return sum;
}
