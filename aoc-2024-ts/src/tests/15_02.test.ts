import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D15 P02`, async () => {
  const ans = await solve("15", "02");
  assertEquals(ans, 1448458);
});
