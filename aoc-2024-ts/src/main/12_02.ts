import { Compass } from "./utils/compass.ts";
import { h1 } from "./utils/grid.ts";
import { to2DMapString } from "./utils/parsers.ts";

export function solve(input: string): number {
  function nbSides(x: number, y: number, plant: string) {
    // check each "corner", with the 2 adjacent elements
    // 0.1
    // .N.
    // 3.2
    let sides = 0;
    for (const corner of [0, 1, 2, 3]) {
      const [tx, ty] = Compass.DIR4[corner];
      const [ux, uy] = Compass.DIR4[(corner + 1) % 4];
      const [vx, vy] = [tx + ux, ty + uy];

      const n1 = garden[h1([x + tx, y + ty])];
      const n2 = garden[h1([x + ux, y + uy])];
      const n3 = garden[h1([x + vx, y + vy])];

      if (
        (n1 !== plant && n2 !== plant) ||
        (n1 === plant && n2 === plant && n3 != plant)
      ) {
        sides += 1;
      }
    }
    return sides;
  }

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
      let sides = 0;

      // dfs
      const stk: [number, number][] = [[i, j]];
      while (stk.length > 0) {
        const [x, y] = stk.pop()!;

        // "visit" node
        area += 1;
        sides += nbSides(x, y, plant);

        for (const [dx, dy] of Compass.DIR4) {
          const adj = h1([x + dx, y + dy]);
          if (visited[adj] || garden[adj] !== plant) {
            continue;
          }

          visited[adj] = true;
          stk.push([x + dx, y + dy]);
        }
      }

      price += area * sides;
    }
  }

  return price;
}
