export function solve(input: string): number {
  const rows = input.trim().split("\n");

  // parse orders
  const orders = new Map<number, number[]>();
  let i = 0;
  while (rows[i].trim().length !== 0) {
    const [a, b] = rows[i].trim().split("|").map((s) => Number.parseInt(s, 10));
    if (!orders.has(a)) {
      orders.set(a, []);
    }
    orders.get(a)?.push(b);
    i += 1;
  }

  // parse updates
  // fix incorrectly-ordered updates
  const fixedUpdates = [];
  i += 1;
  while (i < rows.length && rows[i].trim().length !== 0) {
    const deps = new Set<number>();
    const update = rows[i].trim().split(",").map((s) => Number.parseInt(s, 10));
    let isOrdered = true;
    // starting from the end
    // keep track of all dependencies
    // an update has the right order if
    // older values aren't part of the list
    for (const x of update.reverse()) {
      if (deps.has(x)) {
        isOrdered = false;
        break;
      }
      for (const y of orders.get(x) || []) {
        deps.add(y);
      }
    }

    if (!isOrdered) {
      // fix errors
      update.sort((a, b) => (orders.get(b) || []).indexOf(a));
      fixedUpdates.push(update);
    }

    i += 1;
  }

  // calculate sum
  let sum = 0;
  for (const update of fixedUpdates) {
    sum += update[Math.floor((update.length - 1) / 2)];
  }

  return sum;
}
