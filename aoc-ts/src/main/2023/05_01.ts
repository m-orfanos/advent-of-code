import { to1DArrayNumeric } from "../utils/parsers.ts";

export const SEED_TO_SOIL = "seed-to-soil";
export const SOIL_TO_FERTILIZER = "soil-to-fertilizer";
export const FERTILIZER_TO_WATER = "fertilizer-to-water";
export const WATER_TO_LIGHT = "water-to-light";
export const LIGHT_TO_TEMPERATURE = "light-to-temperature";
export const TEMPERATURE_TO_HUMIDITY = "temperature-to-humidity";
export const HUMIDITY_TO_LOCATION = "humidity-to-location";

export function solve(input: string): number {
  const rows = input.trim().split("\n");
  const seeds = to1DArrayNumeric(rows[0].split(":")[1]);
  const map = parseMappings(rows);

  let ans = Infinity;
  for (const seed of seeds) {
    const soil = next(seed, map[SEED_TO_SOIL]);
    const fertilizer = next(soil, map[SOIL_TO_FERTILIZER]);
    const water = next(fertilizer, map[FERTILIZER_TO_WATER]);
    const light = next(water, map[WATER_TO_LIGHT]);
    const temperature = next(light, map[LIGHT_TO_TEMPERATURE]);
    const humidity = next(temperature, map[TEMPERATURE_TO_HUMIDITY]);
    const location = next(humidity, map[HUMIDITY_TO_LOCATION]);
    ans = Math.min(location, ans);
  }

  return ans;
}

function next(n: number, mapping: [number, number, number][]) {
  let m = n;
  for (const [dst, src, rng] of mapping) {
    if (src <= n && n <= src + rng) {
      m = dst + (n - src);
      break;
    }
  }
  return m;
}

export function parseMappings(rows: string[]) {
  const map: { [key: string]: [number, number, number][] } = {
    [SEED_TO_SOIL]: [],
    [SOIL_TO_FERTILIZER]: [],
    [FERTILIZER_TO_WATER]: [],
    [WATER_TO_LIGHT]: [],
    [LIGHT_TO_TEMPERATURE]: [],
    [TEMPERATURE_TO_HUMIDITY]: [],
    [HUMIDITY_TO_LOCATION]: [],
  };

  let i = 0;
  let curr: [number, number, number][] = [];
  while (i < rows.length) {
    if (rows[i].indexOf(SEED_TO_SOIL) >= 0) {
      curr = map[SEED_TO_SOIL];
      i += 1;
    } else if (rows[i].indexOf(SOIL_TO_FERTILIZER) >= 0) {
      curr = map[SOIL_TO_FERTILIZER];
      i += 1;
    } else if (rows[i].indexOf(FERTILIZER_TO_WATER) >= 0) {
      curr = map[FERTILIZER_TO_WATER];
      i += 1;
    } else if (rows[i].indexOf(WATER_TO_LIGHT) >= 0) {
      curr = map[WATER_TO_LIGHT];
      i += 1;
    } else if (rows[i].indexOf(LIGHT_TO_TEMPERATURE) >= 0) {
      curr = map[LIGHT_TO_TEMPERATURE];
      i += 1;
    } else if (rows[i].indexOf(TEMPERATURE_TO_HUMIDITY) >= 0) {
      curr = map[TEMPERATURE_TO_HUMIDITY];
      i += 1;
    } else if (rows[i].indexOf(HUMIDITY_TO_LOCATION) >= 0) {
      curr = map[HUMIDITY_TO_LOCATION];
      i += 1;
    } else if (rows[i].trim().length === 0) {
      curr = [];
      i += 1;
    } else {
      while (i < rows.length && rows[i].trim().length !== 0) {
        curr.push(to1DArrayNumeric(rows[i]) as [number, number, number]);
        i += 1;
      }
      curr.sort((a, b) => a[1] - b[1]);

      // fill in missing intervals
      const missing: [number, number, number][] = [];
      if (curr[0][1] !== 0) {
        missing.push([0, 0, curr[0][1]]);
      }
      let tmp = curr[0][1];
      for (let j = 0; j < curr.length; j++) {
        if (tmp < curr[j][1]) {
          missing.push([tmp, tmp, curr[j][1] - tmp]);
        }
        tmp = curr[j][1] + curr[j][2];
      }
      missing.push([
        curr[curr.length - 1][1] + curr[curr.length - 1][2],
        curr[curr.length - 1][1] + curr[curr.length - 1][2],
        Infinity,
      ]);
      curr.push(...missing);
    }
  }

  return map;
}
