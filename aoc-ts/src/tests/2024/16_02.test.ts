import { assertEquals } from "jsr:@std/assert";
import { solve } from "../solve.ts";

Deno.test(`2024 D16 P02`, async () => {
  const ans = await solve("2024", "16", "02");
  assertEquals(ans, 494);
});
