import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D09 P02`, async () => {
  const ans = await solve("09", "02");
  assertEquals(ans, 6307279963620);
});
