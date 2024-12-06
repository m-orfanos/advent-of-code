import { assertEquals } from "jsr:@std/assert";
import { solve } from "../solve.ts";

Deno.test(`2023 D06 P02`, async () => {
  const ans = await solve("2023", "06", "02");
  assertEquals(ans, 34655848);
});
