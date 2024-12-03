import { parse } from "https://deno.land/std@0.200.0/flags/mod.ts";
import type { Args } from "https://deno.land/std@0.200.0/flags/mod.ts";
import { exists } from "https://deno.land/std@0.200.0/fs/mod.ts";

await main(Deno.args);

async function main(inputArgs: string[]): Promise<void> {
  // parge args
  const args = parseArguments(inputArgs);

  if (args.help) {
    printHelp();
    Deno.exit(0);
  }

  // TODO: remove hardcoded year someday maybe
  const year = 2024;
  const day = Number.parseInt(args.day, 10);
  const dayStr = (day + "").padStart(2, "0");
  const part: number = Number.parseInt(args.part, 10);
  const partStr = (part + "").padStart(2, "0");

  const example: boolean = args.example;

  // validate args
  if (!day || !part) {
    printHelp();
    Deno.exit(0);
  }

  console.log(`Solving AoC 2024 Day:${dayStr} Part:${partStr}` + (example ? " example" : ""));

  // puzzle input
  const pfx = `${dayStr}_${partStr}`;
  const sfx = example ? "_example" : "";
  const input_filename = `./src/resources/${pfx}${sfx}.txt`;
  const isFileExists = await exists(input_filename);
  if (!isFileExists && !example) {
    const content = await fetchPuzzleInput(year, day);
    await Deno.writeTextFile(input_filename, content);
  }
  const input = await Deno.readTextFile(input_filename);
  console.log("Puzzle input is cached.");

  // solve puzzle
  const puzzle = await import(`./${dayStr}_${partStr}.ts`);
  const solution = puzzle.solve(input);
  console.log(`Answer: ${solution}`);
}

async function fetchPuzzleInput(year: number, day: number): Promise<string> {
  console.log("Fetching puzzle input");
  const resp = await fetch(`https://adventofcode.com/${year}/day/${day}/input`, {
    headers: { cookie: `session=${Deno.env.get("AOC_SESSION")}` },
  });
  if (!resp.ok) {
    console.error(`${resp.status} - ${resp.statusText} - Could not fetch puzzle input`);
    Deno.exit(0);
  }
  return resp.text();
}

function parseArguments(args: string[]): Args {
  const booleanArgs = [
    "help",
    "example",
  ];

  const stringArgs: string[] = [
    // "year",
    "day",
    "part",
  ];

  const alias = {
    "help": "h",
    "example": "e",
    // "year": "y",
    "day": "d",
    "part": "p",
  };

  return parse(args, {
    alias,
    boolean: booleanArgs,
    string: stringArgs,
  });
}

function printHelp(): void {
  console.log(`Usage: aoc [OPTIONS...]`);
  console.log("\nOptional flags:");
  console.log("  -h, --help       Display this help and exit");
  console.log("  -d, --day        The puzzle day (01-25) to solve");
  console.log("  -p, --part       The puzzle part (01, 02) to solve");
}
