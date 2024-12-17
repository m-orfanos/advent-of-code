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
