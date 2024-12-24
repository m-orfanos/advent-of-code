import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D08 P01`, async () => {
  const ans = await solve("08", "01");
  assertEquals(ans, 289);
});
