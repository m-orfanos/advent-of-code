import { checksum, initializeState } from "./09_01.ts";
import { to1DArrayNumeric } from "./utils/parsers.ts";

export function solve(input: string): number {
  // parse input
  const diskMap = to1DArrayNumeric(input, "");
  const { disk, chunks, offsets } = initializeState(diskMap);

  // defragment
  for (let i = 0; i < chunks.length; i++) {
    const fileId = chunks.length - 1 - i;
    const [fileSize, _] = chunks[fileId];
    const idx = findContiguous(disk, fileSize, offsets[fileId]);
    if (idx >= 0) {
      // move file
      disk.fill(Infinity, offsets[fileId], offsets[fileId] + fileSize);
      disk.fill(fileId, idx, idx + fileSize);
    }
  }

  return checksum(disk);
}

function findContiguous(arr: number[], len: number, max: number) {
  let curr = 0;
  for (let i = 0; i < max; i++) {
    if (arr[i] === Infinity) {
      curr += 1;
    } else {
      curr = 0;
    }
    if (curr === len) {
      return i - curr + 1;
    }
  }
  return -1;
}
