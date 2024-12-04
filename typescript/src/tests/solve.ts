export async function solve(year: string, day: string, part: string) {
  async function getPuzzle(year: string, day: string, part: string) {
    return await import(`../main/${year}/${day}_${part}.ts`);
  }

  async function getInput(year: string, day: string, part: string) {
    const input_filename = `./src/resources/${year}/${day}_${part}.txt`;
    const input = await Deno.readTextFile(input_filename);
    return input;
  }

  const input = await getInput(year, day, part);
  const puzzle = await getPuzzle(year, day, part);
  const ans = puzzle.solve(input);

  return ans;
}
