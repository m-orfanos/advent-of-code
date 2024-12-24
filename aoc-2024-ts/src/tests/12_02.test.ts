import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D12 P02`, async () => {
  const ans = await solve("12", "02");
  assertEquals(ans, 901100);
});
