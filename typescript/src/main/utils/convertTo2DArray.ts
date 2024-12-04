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
export function convertTo2DArray(input: string): number[][] {
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
