import { assertEquals } from "jsr:@std/assert";
import { solve } from "../solve.ts";

Deno.test(`2024 D17 P02`, async () => {
  const ans = await solve("2024", "17", "02");
  assertEquals(ans, 190384609508367);
});
