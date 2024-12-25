import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D22 P01`, async () => {
  const ans = await solve("22", "01");
  assertEquals(ans, 16619522798);
});
