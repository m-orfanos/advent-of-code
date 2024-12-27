import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D24 P01`, async () => {
  const ans = await solve("24", "01");
  assertEquals(ans, 64755511006320);
});
