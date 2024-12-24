import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D15 P01`, async () => {
  const ans = await solve("15", "01");
  assertEquals(ans, 1442192);
});
