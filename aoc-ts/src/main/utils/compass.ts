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

function add(u: Vector, v: Vector): Vector {
  return [u[0] + v[0], u[1] + v[1]];
}

// FIXME: needs clarification
export function mul(a: [number, number], b: [number, number]): [number, number] {
  // (a+bi)*(c+di)
  // ac + adi + bci + (-bd)
  // (ac-bd) + (ad+bc)i
  return [(a[0] * b[0]) - (a[1] * b[1]), (a[0] * b[1]) + (a[1] * b[0])];
}

export function isBounded<T>(p: [number, number], grid: T[][]): boolean {
  return 0 <= p[0] && p[0] < grid.length &&
    0 <= p[1] && p[1] < grid[p[0]].length;
}
