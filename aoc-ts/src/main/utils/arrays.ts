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

export function zip<T>(a: T[], b: T[]): T[][] {
  return a.map((k, i) => [k, b[i]]);
}

export function new2DArray<T>(m: number, n: number, supplier: () => T) {
  const arr: T[][] = [];
  for (let i = 0; i < m; i++) {
    if (!arr[i]) {
      arr[i] = [];
    }
    for (let j = 0; j < n; j++) {
      arr[i][j] = supplier();
    }
  }
  return arr;
}
