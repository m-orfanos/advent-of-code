import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D05 P01`, async () => {
  const ans = await solve("05", "01");
  assertEquals(ans, 4957);
});
