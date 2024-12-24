import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D04 P02`, async () => {
  const ans = await solve("04", "02");
  assertEquals(ans, 1866);
});
