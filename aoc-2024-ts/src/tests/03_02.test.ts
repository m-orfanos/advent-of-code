import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D03 P02`, async () => {
  const ans = await solve("03", "02");
  assertEquals(ans, 89798695);
});
