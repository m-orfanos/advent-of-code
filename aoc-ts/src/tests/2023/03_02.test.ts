import { assertEquals } from "jsr:@std/assert";
import { solve } from "../solve.ts";

Deno.test(`2023 D03 P02`, async () => {
  const ans = await solve("2023", "03", "02");
  assertEquals(ans, 84883664);
});
