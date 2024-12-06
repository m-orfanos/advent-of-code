/**
 * Converts the input into a 2D array of numbers.
 *
 * @remarks
 * The input is assumed to follow a row-major order.
 * The result might be a ragged-array.
 *
 * @param input
 * @returns
 */
export function to2DArrayNumeric(input: string): number[][] {
  const grid: number[][] = [];

  const rows = input.trim().split("\n");
  for (const row of rows) {
    const curr: number[] = [];
    grid.push(curr);
    const cols = row.trim().split(" ");
    for (const col of cols) {
      if (col.length == 0) {
        continue;
      }
      curr.push(Number.parseInt(col, 10));
    }
  }

  return grid;
}

export function to2DArrayString(input: string): string[][] {
  const grid: string[][] = [];

  const rows = input.trim().split("\n");
  for (const row of rows) {
    const curr: string[] = [];
    grid.push(curr);
    const cols = row.trim().split("");
    for (const col of cols) {
      if (col.trim().length == 0) {
        continue;
      }
      curr.push(col.trim());
    }
  }

  return grid;
}

/**
 * Converts the input into an array of strings.
 *
 * @remarks
 * The input is assumed to follow a row-major order.
 * The result might be a ragged-array.
 *
 * @param input
 * @returns
 */
export function toArrayString(input: string): string[] {
  const grid: string[] = [];
  const rows = input.trim().split("\n");
  for (const row of rows) {
    const cols = row.trim().split(" ");
    for (const col of cols) {
      if (col.length == 0) {
        continue;
      }
      grid.push(col);
    }
  }

  return grid;
}

/**
 * Converts a string into an array of numbers.
 *
 * Example
 *   Input : "83 86  6 31 17  9 48 53"
 *   Output: [83, 86, 6, 31, 17, 9, 48, 53]
 */
export function toArrayNumeric(str: string): number[] {
  return str.trim()
    .split(" ")
    .map((x) => x.trim())
    .filter((x) => x.length > 0)
    .map((x) => Number.parseInt(x, 10));
}
