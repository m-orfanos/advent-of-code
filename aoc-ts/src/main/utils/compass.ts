export type Vector = [number, number];
export type Complex = [number, number];

export class Compass {
  // main directions
  static readonly NORTH: Vector = [-1, 0];
  static readonly EAST: Vector = [0, 1];
  static readonly SOUTH: Vector = [1, 0];
  static readonly WEST: Vector = [0, -1];

  // ordinal directions
  static readonly NORTH_EAST = add(this.NORTH, this.EAST);
  static readonly SOUTH_EAST = add(this.SOUTH, this.EAST);
  static readonly SOUTH_WEST = add(this.SOUTH, this.WEST);
  static readonly NORTH_WEST = add(this.NORTH, this.WEST);

  // main directions
  static readonly DIR4 = [
    this.NORTH,
    this.EAST,
    this.SOUTH,
    this.WEST,
  ];

  // main + ordinal directions
  static readonly DIR8 = [
    this.NORTH,
    this.NORTH_EAST,
    this.EAST,
    this.SOUTH_EAST,
    this.SOUTH,
    this.SOUTH_WEST,
    this.WEST,
    this.NORTH_WEST,
  ];
}

export function rev(d: [number, number]): [number, number] {
  return mul(d, [-1, 0]);
}

export function eq(u: [number, number], v: [number, number]): boolean {
  return u[0] == v[0] && u[1] == v[1];
}

export function add(u: [number, number], v: [number, number]): [number, number] {
  return [u[0] + v[0], u[1] + v[1]];
}

export function sub(u: [number, number], v: [number, number]): [number, number] {
  return [u[0] - v[0], u[1] - v[1]];
}

export function mul(a: [number, number], b: [number, number]): [number, number] {
  // |  ax |  ay |     |  bx |  by |     |   x |   y | notes                 |
  // | ---:| ---:| --- | ---:| ---:| --- | ---:| ---:| --------------------- |
  // |   0 |   1 |     |   0 |   1 |     |  -1 |   0 | 90d counter-clockwise |
  // |   0 |  -1 |     |   0 |   1 |     |   1 |   0 |                       |
  // |   1 |   0 |     |   0 |   1 |     |   0 |   1 |                       |
  // |  -1 |   0 |     |   0 |   1 |     |   0 |  -1 |                       |
  // |     |     |     |     |     |     |     |     |                       |
  // |   0 |   1 |     |   0 |  -1 |     |   1 |   0 | 90d clockwise         |
  // |   0 |  -1 |     |   0 |  -1 |     |  -1 |   0 |                       |
  // |   1 |   0 |     |   0 |  -1 |     |   0 |  -1 |                       |
  // |  -1 |   0 |     |   0 |  -1 |     |   0 |   1 |                       |
  // |     |     |     |     |     |     |     |     |                       |
  // |   0 |   1 |     |   1 |   0 |     |   0 |   1 | 0d                    |
  // |   0 |  -1 |     |   1 |   0 |     |   0 |  -1 |                       |
  // |   1 |   0 |     |   1 |   0 |     |   1 |   0 |                       |
  // |  -1 |   0 |     |   1 |   0 |     |  -1 |   0 |                       |
  // |     |     |     |     |     |     |     |     |                       |
  // |   0 |   1 |     |  -1 |   0 |     |   0 |  -1 | 180d                  |
  // |   0 |  -1 |     |  -1 |   0 |     |   0 |   1 |                       |
  // |   1 |   0 |     |  -1 |   0 |     |  -1 |   0 |                       |
  // |  -1 |   0 |     |  -1 |   0 |     |   1 |  -0 |                       |
  return [(a[0] * b[0]) - (a[1] * b[1]), (a[0] * b[1]) + (a[1] * b[0])];
}

export function isBounded<T>(p: [number, number], grid: T[][]): boolean {
  return 0 <= p[0] && p[0] < grid.length &&
    0 <= p[1] && p[1] < grid[p[0]].length;
}

export function h1(a: [number, number]) {
  return a.join("|");
}

export function h2(a: [number, number], b: [number, number]) {
  return [...a, ...b].join("|");
}
