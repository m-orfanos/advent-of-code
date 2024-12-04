export function solve(input: string): number {
  let calibration = 0;
  let first = 0;
  let last = 0;

  const rows = input.trim().split("\n");
  for (const row of rows) {
    for (const ch of row.trim().split("")) {
      if (!Number.isNaN(Number.parseInt(ch, 10))) {
        first = Number.parseInt(ch, 10);
        break;
      }
    }

    for (const ch of row.trim().split("").reverse()) {
      if (!Number.isNaN(Number.parseInt(ch, 10))) {
        last = Number.parseInt(ch, 10);
        break;
      }
    }

    calibration += first * 10 + last;
  }

  return calibration;
}
