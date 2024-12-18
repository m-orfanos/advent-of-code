import { assertEquals } from "jsr:@std/assert";
import { solve } from "../solve.ts";

Deno.test(`2024 D11 P01`, async () => {
  const ans = await solve("2024", "11", "01");
  assertEquals(ans, 186424);
});
