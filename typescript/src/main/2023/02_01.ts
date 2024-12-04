export function solve(input: string): number {
  const limits: { [key: string]: number } = {
    "red": 12,
    "green": 13,
    "blue": 14,
  };

  let cnt = 0;
  const rows = input.trim().split("\n");
  for (let i = 0; i < rows.length; i++) {
    const a = rows[i].trim().split(":");
    const rounds = a[1].trim().split(";");
    let isPossible = true;
    for (const b of rounds) {
      if (!isPossible) {
        break;
      }
      const round = b.trim().split(",");
      for (const c of round) {
        const event = c.trim().split(" ");
        const n = Number.parseInt(event[0].trim(), 10);
        const colour = event[1].trim();
        if (n > limits[colour]) {
          isPossible = false;
          break;
        }
      }
    }

    if (isPossible) {
      cnt += i + 1;
    }
  }

  return cnt;
}
