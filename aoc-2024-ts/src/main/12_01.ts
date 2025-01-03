import { Compass } from "./utils/compass.ts";
import { h1 } from "./utils/grid.ts";
import { to2DMapString } from "./utils/parsers.ts";

export function solve(input: string): number {
  const [garden, nr, nc] = to2DMapString(input);
  const visited: { [key: string]: boolean } = {};

  let price = 0;
  for (let i = 0; i < nr; i++) {
    for (let j = 0; j < nc; j++) {
      const cur = h1([i, j]);
      if (visited[cur]) {
        continue;
      }
      visited[cur] = true;

      const plant = garden[cur];
      let area = 0;
      let perimeter = 0;

      const stk = [[i, j]];
      while (stk.length > 0) {
        const [x, y] = stk.pop()!;
        area += 1;

        for (const [dx, dy] of Compass.DIR4) {
          const adj = h1([x + dx, y + dy]);
          if (garden[adj] !== plant) {
            perimeter += 1;
            continue;
          }

          if (visited[adj]) {
            continue;
          }
          visited[adj] = true;

          stk.push([x + dx, y + dy]);
        }
      }

      price += area * perimeter;
    }
  }

  return price;
}
