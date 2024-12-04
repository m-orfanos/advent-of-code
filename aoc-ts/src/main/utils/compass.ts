export class Direction {
  // main directions
  static readonly NORTH = new Direction(-1, 0);
  static readonly EAST = new Direction(0, 1);
  static readonly SOUTH = new Direction(1, 0);
  static readonly WEST = new Direction(0, -1);
  // ordinal directions
  static readonly NORTH_EAST = this.add(this.NORTH, this.EAST);
  static readonly SOUTH_EAST = this.add(this.SOUTH, this.EAST);
  static readonly SOUTH_WEST = this.add(this.SOUTH, this.WEST);
  static readonly NORTH_WEST = this.add(this.NORTH, this.WEST);

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

  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }

  static add(d1: Direction, d2: Direction) {
    return new Direction(d1.x + d2.x, d1.y + d2.y);
  }
}

export class Point {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}
