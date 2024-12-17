import { to1DArrayNumeric } from "../utils/parsers.ts";
import { chunk } from "../utils/arrays.ts";
import {
  FERTILIZER_TO_WATER,
  HUMIDITY_TO_LOCATION,
  LIGHT_TO_TEMPERATURE,
  parseMappings,
  SEED_TO_SOIL,
  SOIL_TO_FERTILIZER,
  TEMPERATURE_TO_HUMIDITY,
  WATER_TO_LIGHT,
} from "./05_01.ts";

export function solve(input: string): number {
  const rows = input.trim().split("\n");
  const seeds = to1DArrayNumeric(rows[0].split(":")[1]);
  const chunks = chunk(seeds, 2) as [number, number][];
  const map = parseMappings(rows);

  const soil = next(chunks, map[SEED_TO_SOIL]);
  const fertilizer = next(soil, map[SOIL_TO_FERTILIZER]);
  const water = next(fertilizer, map[FERTILIZER_TO_WATER]);
  const light = next(water, map[WATER_TO_LIGHT]);
  const temperature = next(light, map[LIGHT_TO_TEMPERATURE]);
  const humidity = next(temperature, map[TEMPERATURE_TO_HUMIDITY]);
  const location = next(humidity, map[HUMIDITY_TO_LOCATION]);

  location.sort((a, b) => a[0] - b[0]);

  return location[0][0];
}

function next(
  sources: [number, number][],
  mapping: [number, number, number][],
): [number, number][] {
  const overlaps: [number, number][] = [];
  for (const [src1, rng1] of sources) {
    for (const [dst, src2, rng2] of mapping) {
      const lhs = Math.max(src1, src2);
      const rhs = Math.min(src1 + rng1, src2 + rng2);
      if (lhs <= rhs) {
        overlaps.push([lhs + (dst - src2), rhs - lhs]);
      }
    }
  }
  return overlaps;
}
