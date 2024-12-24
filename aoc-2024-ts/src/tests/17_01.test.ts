import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D17 P01`, async () => {
  const ans = await solve("17", "01");
  assertEquals(ans, "2,3,4,7,5,7,3,0,7");
});
