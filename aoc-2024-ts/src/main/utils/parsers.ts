import { h1 } from "./compass.ts";

/**
 * Converts a string into an array of numbers.
 */
export function to1DArrayNumeric(str: string, delimiter = " "): number[] {
  return str.trim()
    .split(delimiter)
    .map((x) => x.trim())
    .filter((x) => x.length > 0)
    .map((x) => Number.parseInt(x, 10));
}

/**
 * Converts the input into a 2D array of numbers.
 */
export function to2DArrayNumeric(input: string, delimiter = " "): number[][] {
  return input.trim()
    .split("\n")
    .map((r) =>
      r.trim()
        .split(delimiter)
        .map((c) => c.trim())
        .map((c) => Number.parseInt(c, 10))
    );
}

/**
 * Converts the input into an array of strings.
 */
export function to1DArrayString(input: string): string[] {
  return input.trim()
    .split("\n")
    .map((x) => x.trim());
}

/**
 * Converts the input into a 2D array of characters.
 */
export function to2DArrayString(input: string): string[][] {
  return input.trim()
    .split("\n")
    .map((r) =>
      r.trim()
        .split("")
    );
}

/**
 * Converts the input into a map of characters.
 */
export function to2DMapString(input: string): [{ [key: string]: string }, number, number] {
  const grid = to2DArrayString(input);
  const ans: { [key: string]: string } = {};
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      ans[h1([i, j])] = grid[i][j];
    }
  }

  return [ans, grid.length, grid[0].length];
}

export function find(needle: string, grid: string[][]): [number, number] | null {
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[i].length; j++) {
      if (grid[i][j] === needle) {
        return [i, j];
      }
    }
  }
  return null;
}
