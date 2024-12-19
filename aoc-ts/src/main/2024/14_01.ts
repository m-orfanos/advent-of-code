import { to1DArrayString } from "../utils/parsers.ts";

export function solve(input: string): number {
  // magic numbers not part of the input
  const t = 100;
  const w = 101;
  const h = 103;

  const robots = parse(input);
  simulate(t, w, h, robots);
  return safety(w, h, robots);
}

export function parse(input: string): number[][][] {
  const rows = to1DArrayString(input);
  const robots = rows.map((x) =>
    x.split(" ")
      .map((x) =>
        x.trim().split("=")[1]
          .split(",")
          .map((x) => Number.parseInt(x.trim(), 10))
      )
  );
  return robots;
}

export function simulate(
  t: number,
  w: number,
  h: number,
  robots: number[][][],
): void {
  for (let i = 0; i < t; i++) {
    for (const r of robots) {
      const [y, x] = r[0];
      const [vy, vx] = r[1];
      r[0][0] = (((y + vy) % w) + w) % w;
      r[0][1] = (((x + vx) % h) + h) % h;
    }
  }
}

function safety(w: number, h: number, robots: number[][][]): number {
  const wf = Math.floor(w / 2);
  const wc = Math.ceil(w / 2);
  const hf = Math.floor(h / 2);
  const hc = Math.ceil(h / 2);

  const quadrants = [
    [[0, wf], [0, hf]],
    [[0, wf], [hc, h]],
    [[wc, w], [0, hf]],
    [[wc, w], [hc, h]],
  ];

  const nbRobots = [0, 0, 0, 0];
  for (const r of robots) {
    for (let i = 0; i < quadrants.length; i++) {
      const [[a, b], [c, d]] = quadrants[i];
      if (
        a <= r[0][0] && r[0][0] < b &&
        c <= r[0][1] && r[0][1] < d
      ) {
        nbRobots[i] += 1;
      }
    }
  }

  return nbRobots.reduce((acc, curr) => acc * curr, 1);
}

export function print(
  w: number,
  h: number,
  robots: number[][][],
): void {
  for (let i = 0; i < h; i++) {
    let pr = "";
    for (let j = 0; j < w; j++) {
      let cnt = 0;
      for (let k = 0; k < robots.length; k++) {
        const r = robots[k];
        if (r[0][0] == j && r[0][1] == i) {
          cnt += 1;
        }
      }
      if (cnt > 0) {
        pr += cnt;
      } else {
        pr += ".";
      }
    }
    console.log(pr);
  }
  console.log();
}
