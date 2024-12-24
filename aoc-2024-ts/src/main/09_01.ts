import { chunk, new1DArray } from "./utils/arrays.ts";
import { to1DArrayNumeric } from "./utils/parsers.ts";

export function solve(input: string): number {
  // parse input
  const diskMap = to1DArrayNumeric(input, "");
  const { disk } = initializeState(diskMap);

  // defragment using a 2 pointer approach
  let lhs = 0;
  let rhs = disk.length - 1;
  while (lhs < rhs) {
    if (disk[lhs] === Infinity && disk[rhs] !== Infinity) {
      // swap
      disk[lhs] = disk[rhs];
      disk[rhs] = Infinity;
      lhs += 1;
      rhs -= 1;
    } else if (disk[lhs] !== Infinity) {
      lhs += 1;
    } else {
      rhs -= 1;
    }
  }

  return checksum(disk);
}

export function initializeState(diskMap: number[]) {
  const diskSize = diskMap.reduce((acc, curr) => acc + curr, 0);
  const disk = new1DArray(diskSize, () => Infinity);
  const chunks = chunk(diskMap, 2);
  const offsets = [];

  let curr = 0;
  for (let i = 0; i < chunks.length; i++) {
    const [fileSize, freeSpace] = chunks[i];
    offsets.push(curr);
    for (let j = 0; j < fileSize; j++) {
      disk[curr] = i;
      curr += 1;
    }
    curr += freeSpace;
  }

  return { disk, chunks, offsets };
}

export function checksum(arr: number[]) {
  let sum = 0;
  for (let i = 0; i < arr.length; i++) {
    if (arr[i] === Infinity) {
      continue;
    }
    sum += i * arr[i];
  }
  return sum;
}
