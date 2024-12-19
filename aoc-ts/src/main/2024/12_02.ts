import { Direction } from "../utils/compass.ts";
import { hash, to2DMapString } from "../utils/parsers.ts";

export function solve(input: string): number {
  function nbSides(x: number, y: number, plant: string) {
    // check each "corner", with the 2 adjacent elements
    // 0.1
    // .N.
    // 3.2
    let sides = 0;
    for (const corner of [0, 1, 2, 3]) {
      const d1 = Direction.DIR4[corner];
      const d2 = Direction.DIR4[(corner + 1) % 4];
      const d3 = Direction.add(d1, d2);

      const n1 = garden[hash(x + d1.x, y + d1.y)];
      const n2 = garden[hash(x + d2.x, y + d2.y)];
      const n3 = garden[hash(x + d3.x, y + d3.y)];

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
      const cur = hash(i, j);
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

        for (const d of Direction.DIR4) {
          const adj = hash(x + d.x, y + d.y);
          if (visited[adj] || garden[adj] !== plant) {
            continue;
          }

          visited[adj] = true;
          stk.push([x + d.x, y + d.y]);
        }
      }

      price += area * sides;
    }
  }

  return price;
}
