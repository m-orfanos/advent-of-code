/**
 * Knuth–Morris–Pratt string-searching algorithm
 *
 * https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm
 * https://www.geeksforgeeks.org/kmp-algorithm-for-pattern-searching/
 *
 * Example
 * > kmpSearch("ABC ABCDAB ABCDABCDABDE", "ABC")
 * > [ 0, 4, 11, 15 ]
 *
 * @param txt The string to search
 * @param pat The substring to search for in the string
 * @returns a list of indices where a match was found
 */
export function kmpSearch(txt: string, pat: string): number[] {
  function compute_lps(pat: string): number[] {
    const lps: number[] = Array<number>(pat.length).fill(0);

    let i = 1;
    let l = 0;
    while (i < pat.length) {
      if (pat[i] === pat[l]) {
        l += 1;
        lps[i] = l;
        i += 1;
      } else {
        if (l !== 0) {
          l = lps[l - 1];
        } else {
          lps[i] = 0;
          i += 1;
        }
      }
    }

    return lps;
  }

  const n = txt.length;
  const m = pat.length;

  const matches: number[] = [];
  const lps: number[] = compute_lps(pat);

  let i = 0;
  let j = 0;
  while (i < n) {
    if (txt[i] === pat[j]) {
      i += 1;
      j += 1;
      if (j === m) {
        matches.push(i - j);
        j = lps[j - 1];
      }
    } else {
      if (j !== 0) {
        j = lps[j - 1];
      } else {
        i += 1;
      }
    }
  }

  return matches;
}
