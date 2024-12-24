/**
 * Insanely scuffed "priority queue" :D
 */
export class PriorityQueue<T> {
  data: [number, T][];
  isSorted: boolean;

  constructor() {
    this.data = [];
    this.isSorted = false;
  }

  isEmpty(): boolean {
    return this.data.length === 0;
  }

  push(priority: number, item: T): void {
    this.data.push([priority, item]);
    this.isSorted = false;
  }

  pop(): T {
    if (!this.isSorted) {
      this.data.sort((a, b) => b[0] - a[0]);
      this.isSorted = true;
    }
    return this.data.pop()![1];
  }
}
