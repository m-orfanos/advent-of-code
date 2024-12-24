import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D19 P01`, async () => {
  const ans = await solve("19", "01");
  assertEquals(ans, 263);
});
