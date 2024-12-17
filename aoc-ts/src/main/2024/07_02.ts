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
    // to calculate add/mul/con
    let i = 1;
    let calcs: number[] = [xs[0]];
    while (i < xs.length) {
      const tmp = [];
      while (calcs.length > 0) {
        const curr = calcs.pop()!;

        const add = curr + xs[i];
        const mul = curr * xs[i];
        const con = Number.parseInt(`${curr}${xs[i]}`, 10);

        if (add <= val) {
          tmp.push(add);
        }
        if (mul <= val) {
          tmp.push(mul);
        }
        if (con <= val) {
          tmp.push(con);
        }
      }

      i += 1;
      calcs = tmp;
    }

    for (const cal of calcs) {
      if (cal === val) {
        sum += val;
        break;
      }
    }
  }

  return sum;
}
