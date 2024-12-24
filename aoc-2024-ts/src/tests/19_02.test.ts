import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D19 P02`, async () => {
  const ans = await solve("19", "02");
  assertEquals(ans, 723524534506343);
});
