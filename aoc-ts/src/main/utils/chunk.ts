export function chunk(xs: number[], n: number): number[][] {
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
