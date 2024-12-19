import { assertEquals } from "jsr:@std/assert";
import { solve } from "../solve.ts";

Deno.test(`2024 D12 P01`, async () => {
  const ans = await solve("2024", "12", "01");
  assertEquals(ans, 1473276);
});
