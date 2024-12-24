import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D07 P02`, async () => {
  const ans = await solve("07", "02");
  assertEquals(ans, 348360680516005);
});
