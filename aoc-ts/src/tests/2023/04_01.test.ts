import { assertEquals } from "jsr:@std/assert";
import { solve } from "../solve.ts";

Deno.test(`2023 D04 P01`, async () => {
  const ans = await solve("2023", "04", "01");
  assertEquals(ans, 20107);
});
