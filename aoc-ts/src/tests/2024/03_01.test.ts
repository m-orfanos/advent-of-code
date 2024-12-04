import { assertEquals } from "jsr:@std/assert";
import { solve } from "../solve.ts";

Deno.test(`2024 D03 P01`, async () => {
  const ans = await solve("2024", "03", "01");
  assertEquals(ans, 185797128);
});