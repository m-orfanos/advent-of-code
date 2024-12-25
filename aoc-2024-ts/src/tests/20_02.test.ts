import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D20 P02`, async () => {
  const ans = await solve("20", "02");
  assertEquals(ans, 1006850);
});
