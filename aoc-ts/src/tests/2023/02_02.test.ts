import { assertEquals } from "jsr:@std/assert";
import { solve } from "../solve.ts";

Deno.test(`2023 D02 P02`, async () => {
  const ans = await solve("2023", "02", "02");
  assertEquals(ans, 70924);
});
