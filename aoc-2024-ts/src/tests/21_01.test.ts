import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D21 P01`, async () => {
  const ans = await solve("21", "01");
  assertEquals(ans, 248684);
});
