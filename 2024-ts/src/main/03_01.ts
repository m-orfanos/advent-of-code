export function solve(input: string): number {
  const regex = /(mul\((\d+),(\d+)\))/g;
  let m;
  let ans = 0;
  while ((m = regex.exec(input)) !== null) {
    ans += Number.parseInt(m[2], 10) * Number.parseInt(m[3], 10);
  }
  return ans;
}
