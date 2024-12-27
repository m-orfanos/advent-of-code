/**
 * Splits a list into equally-sized chunks `n`.
 *
 * The last chunk may contain less than `n` elements.
 */
export function chunk<T>(xs: T[], n: number): T[][] {
  const chunks = [];
  for (let i = 0; i < xs.length; i += n) {
    const tmp = [];
    for (let j = 0; j < n; j++) {
      tmp.push(xs[i + j]);
    }
    chunks.push(tmp);
  }
  return chunks;
}

/**
 * Iterates over 2 arrays in parallel, producing tuples with an item from each one.
 *
 * If the lengths of the arrays are different, the remaining elements from
 * longer array are ignored.
 */
export function zip<A, B>(a: A[], b: B[]): [A, B][] {
  const size = a.length < b.length ? a.length : b.length;
  const arr: [A, B][] = [];
  for (let i = 0; i < size; i++) {
    arr.push([a[i], b[i]]);
  }
  return arr;
}

/**
 * Returns a new array of size `n`.
 */
export function new1DArray<T>(n: number, supplier: () => T): T[] {
  return Array(n).fill(supplier());
}

/**
 * Returns a new array of size `mxn`.
 */
export function new2DArray<T>(m: number, n: number, supplier: () => T): T[][] {
  const arr: T[][] = [];
  for (let i = 0; i < m; i++) {
    arr.push(Array(n).fill(supplier()));
  }
  return arr;
}

export function sum(arr: number[]): number {
  return arr.reduce((acc, curr) => acc + curr, 0);
}

export function average(arr: number[]): number {
  return sum(arr) / arr.length;
}

export function variance(arr: number[]): number {
  let sum = 0;
  const avg = average(arr);
  for (const n of arr) {
    sum += (n - avg) * (n - avg);
  }
  return sum / arr.length;
}

export function range(n: number) {
  return [...Array(n).keys()];
}
