import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D09 P01`, async () => {
  const ans = await solve("09", "01");
  assertEquals(ans, 6291146824486);
});
