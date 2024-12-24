export async function solve(day: string, part: string) {
  async function getPuzzle(day: string, part: string) {
    return await import(`../main/${day}_${part}.ts`);
  }

  async function getInput(day: string, part: string) {
    const input_filename = `./src/resources/${day}_${part}.txt`;
    const input = await Deno.readTextFile(input_filename);
    return input;
  }

  const input = await getInput(day, part);
  const puzzle = await getPuzzle(day, part);
  const ans = puzzle.solve(input);

  return ans;
}
