export function solve(input: string): number {
  const rows = input.trim().split("\n");
  let sum = 0;
  for (const row of rows) {
    // parse input
    const tmp = row.split(":").map((x) => x.trim());
    const val = Number.parseInt(tmp[0], 10);
    const xs = tmp[1].split(" ")
      .map((x) => x.trim())
      .map((x) => Number.parseInt(x, 10));

    // perform "DFS", but at every "node" create a split
    // to calculate add/mul
    let i = 1;
    let cals: number[] = [xs[0]];
    while (i < xs.length) {
      const tmp = [];
      while (cals.length > 0) {
        const curr = cals.pop()!;

        const add = curr + xs[i];
        const mul = curr * xs[i];

        if (add <= val) {
          tmp.push(add);
        }
        if (mul <= val) {
          tmp.push(mul);
        }
      }

      i += 1;
      cals = tmp;
    }

    for (const cal of cals) {
      if (cal === val) {
        sum += val;
        break;
      }
    }
  }

  return sum;
}
