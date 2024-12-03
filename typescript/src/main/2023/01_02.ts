export function solve(input: string): number {
  const space: [string, number][] = [
    ["0", 0],
    ["1", 1],
    ["2", 2],
    ["3", 3],
    ["4", 4],
    ["5", 5],
    ["6", 6],
    ["7", 7],
    ["8", 8],
    ["9", 9],
    ["zero", 0],
    ["one", 1],
    ["two", 2],
    ["three", 3],
    ["four", 4],
    ["five", 5],
    ["six", 6],
    ["seven", 7],
    ["eight", 8],
    ["nine", 9],
  ];

  let calibration = 0;
  const rows = input.trim().split("\n");
  for (const row of rows) {
    let ilhs = row.length;
    let vlhs = 0;

    let irhs = 0;
    let vrhs = 0;

    for (const [needle, value] of space) {
      const first = row.slice(0, ilhs + 1).indexOf(needle);
      if (first >= 0) {
        ilhs = first;
        vlhs = value;
      }

      const last = row.slice(irhs).lastIndexOf(needle);
      if (last >= 0) {
        irhs = irhs + last;
        vrhs = value;
      }
    }

    calibration += vlhs * 10 + vrhs;
  }

  return calibration;
}
