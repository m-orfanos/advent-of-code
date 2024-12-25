export function combinations<T>(arr: T[], k: number): T[][] {
  const xss = _combinations(arr.length, k);
  const ans = [];
  for (const xs of xss) {
    const tmp = [];
    for (const x of xs) {
      tmp.push(arr[x]);
    }
    ans.push(tmp);
  }
  return ans;
}

/**
 * Returns an array containing C(n,k) combinations in lexicographical order (McCaffrey).
 *
 * Based on links below:
 * - https://stackoverflow.com/questions/127704/algorithm-to-return-all-combinations-of-k-elements-from-n
 * - https://web.archive.org/web/20170325012457/https://msdn.microsoft.com/en-us/library/aa289166.aspx
 */
function _combinations(n: number, k: number): number[][] {
  function fact(n: number) {
    if (mfacts[n]) {
      return mfacts[n];
    }
    let ans = 1;
    for (let i = 1; i <= n; i++) {
      ans *= i;
    }
    mfacts[n] = ans;
    return ans;
  }

  function comb(n: number, k: number) {
    if (n < k) {
      return 0;
    }
    if (n == k) {
      return 1;
    }
    if (mcombs[`${n}|${k}`]) {
      return mcombs[`${n}|${k}`];
    }
    const tmp = fact(n) / (fact(k) * fact(n - k));
    mcombs[`${n}|${k}`] = tmp;
    return tmp;
  }

  function largest(arr: number[], needle: number) {
    let idx = -1;
    for (let i = 0; i < arr.length; i++) {
      if (needle >= arr[arr.length - 1 - i]) {
        idx = arr.length - 1 - i;
        break;
      }
    }
    return idx;
  }

  const mfacts: { [key: number]: number } = {};
  const mcombs: { [key: string]: number } = {};
  const mtbl: number[][] = [];

  for (let j = 0; j < k; j++) {
    const curr = [];
    for (let i = 0; i < n; i++) {
      curr.push(comb(i, j + 1));
    }
    mtbl.push(curr);
  }

  let i = 0;
  const max = comb(n, k);
  const ans = [];
  while (i < max) {
    let curr = i;
    const tmp = [];
    for (let j = k; j > 0; j--) {
      const ans = largest(mtbl[j - 1], curr);
      tmp.push(ans);
      curr -= mtbl[j - 1][ans];
    }
    ans.push(tmp);
    i += 1;
  }

  return ans;
}
