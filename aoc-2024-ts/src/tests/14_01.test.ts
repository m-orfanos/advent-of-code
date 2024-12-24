import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D14 P01`, async () => {
  const ans = await solve("14", "01");
  assertEquals(ans, 230172768);
});
