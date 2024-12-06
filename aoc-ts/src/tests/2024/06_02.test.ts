import { assertEquals } from "jsr:@std/assert";
import { solve } from "../solve.ts";

Deno.test(`2024 D06 P02`, async () => {
  const ans = await solve("2024", "06", "02");
  assertEquals(ans, 1530);
});
