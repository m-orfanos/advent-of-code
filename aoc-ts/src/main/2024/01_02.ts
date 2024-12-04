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

  const counter = rhs.reduce((acc, curr) => ({
    ...acc,
    [curr]: 1 + (acc[curr] || 0),
  }), {} as Record<number, number>);

  return lhs.reduce((acc, curr) => acc + curr * (counter[curr] || 0), 0);
}
