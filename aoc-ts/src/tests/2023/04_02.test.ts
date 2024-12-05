import { assertEquals } from "jsr:@std/assert";
import { solve } from "../solve.ts";

Deno.test(`2023 D04 P02`, async () => {
  const ans = await solve("2023", "04", "02");
  assertEquals(ans, 8172507);
});
