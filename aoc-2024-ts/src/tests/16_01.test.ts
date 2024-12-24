import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D16 P01`, async () => {
  const ans = await solve("16", "01");
  assertEquals(ans, 91464);
});
