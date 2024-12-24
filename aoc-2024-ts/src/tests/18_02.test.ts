import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D18 P02`, async () => {
  const ans = await solve("18", "02");
  assertEquals(ans, [41, 26]);
});
