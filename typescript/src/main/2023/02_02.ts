export function solve(input: string): number {
  let power = 0;

  const rows = input.trim().split("\n");
  for (let i = 0; i < rows.length; i++) {
    const a = rows[i].trim().split(":");
    const rounds = a[1].trim().split(";");

    const bag: { [key: string]: number } = {
      red: 0,
      green: 0,
      blue: 0,
    };

    for (const b of rounds) {
      const round = b.trim().split(",");
      for (const c of round) {
        const event = c.trim().split(" ");
        const n = Number.parseInt(event[0].trim(), 10);
        const colour = event[1].trim();
        bag[colour] = Math.max(bag[colour], n);
      }
    }

    power += bag["red"] * bag["green"] * bag["blue"];
  }

  return power;
}
