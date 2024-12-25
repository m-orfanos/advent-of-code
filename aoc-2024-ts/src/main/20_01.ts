import { dijkstra, manhattanDistance } from "./utils/grid.ts";
import { find, to2DArrayString } from "./utils/parsers.ts";

export function solve(input: string): number {
  return nbCheats(input, 100, 2);
}

export function nbCheats(
  input: string,
  tSave: number,
  nbPhasing: number,
) {
  const grid = to2DArrayString(input);
  const start = find("S", grid)!;
  const end = find("E", grid)!;

  const path = dijkstra(start, end, grid)[0];

  // the ONLY reason this works is because the programs
  // MUST be back on the normal track again after a cheat
  // meaning the start and end point of a cheat MUST be on
  // the track (i.e. shortest path)
  let cnt = 0;
  for (let i = 0; i < path.length; i++) {
    for (let j = i + 1 + tSave; j < path.length; j++) {
      const md = manhattanDistance(path[i], path[j]);
      if (j - i - md >= tSave && md <= nbPhasing) {
        cnt += 1;
      }
    }
  }
  return cnt;
}
