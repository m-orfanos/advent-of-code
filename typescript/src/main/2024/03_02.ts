export function solve(input: string): number {
  // parse input 3 times
  const segments = [
    ...parseMuls(input),
    ...parseEnableFlags(input),
    ...parseDisableFlags(input),
  ];

  segments.sort((a, b) => a.index - b.index);

  let ans = 0;
  let mode = Mode.START;
  for (const segment of segments) {
    if (mode === Mode.START) {
      if (segment.mode === Mode.ADD) {
        ans += segment.value;
      } else if (segment.mode === Mode.STOP) {
        mode = Mode.STOP;
      }
    } else if (mode === Mode.STOP) {
      // drop segments until the mode is swapped
      if (segment.mode === Mode.START) {
        mode = Mode.START;
      }
    }
  }

  return ans;
}

class Segment {
  index: number;
  mode: Mode;
  value: number;
  constructor(index: number, mode: Mode, value: number) {
    this.index = index;
    this.mode = mode;
    this.value = value;
  }
}

enum Mode {
  START,
  STOP,
  ADD,
}

function parseEnableFlags(input: string): Segment[] {
  const regex = /(do\(\))/g;
  let m;
  const segments: Segment[] = [];
  while ((m = regex.exec(input)) !== null) {
    const idx = m.index;
    const mode = Mode.START;
    segments.push(new Segment(idx, mode, NaN));
  }
  return segments;
}

function parseDisableFlags(input: string): Segment[] {
  const regex = /(don't\(\))/g;
  let m;
  const segments = [];
  while ((m = regex.exec(input)) !== null) {
    const idx = m.index;
    const mode = Mode.STOP;
    segments.push(new Segment(idx, mode, NaN));
  }
  return segments;
}

function parseMuls(input: string): Segment[] {
  const regex = /(mul\((\d+),(\d+)\))/g;
  let m;
  const segments = [];
  while ((m = regex.exec(input)) !== null) {
    const idx = m.index;
    const mode = Mode.ADD;
    const n = Number.parseInt(m[2], 10) * Number.parseInt(m[3], 10);
    segments.push(new Segment(idx, mode, n));
  }
  return segments;
}
