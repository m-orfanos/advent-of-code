import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D22 P02`, async () => {
  const ans = await solve("22", "02");
  assertEquals(ans, 1854);
});
