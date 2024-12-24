import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D01 P01`, async () => {
  const ans = await solve("01", "01");
  assertEquals(ans, 1590491);
});
