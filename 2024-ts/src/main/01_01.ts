export function solve(input: string): number {
  const lhs: number[] = [];
  const rhs: number[] = [];

  const rows = input.trim().split("\n");
  for (const row of rows) {
    const groups = /(\d+)\s+(\d+)/.exec(row)!;
    const left = groups[1];
    const right = groups[2];
    lhs.push(Number.parseInt(left, 10));
    rhs.push(Number.parseInt(right, 10));
  }

  lhs.sort();
  rhs.sort();

  let distance = 0;
  for (let i = 0; i < rows.length; i++) {
    const left = lhs[i];
    const right = rhs[i];
    distance += Math.abs(left - right);
  }

  return distance;
}
