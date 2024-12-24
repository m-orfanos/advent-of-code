import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D10 P02`, async () => {
  const ans = await solve("10", "02");
  assertEquals(ans, 1960);
});
