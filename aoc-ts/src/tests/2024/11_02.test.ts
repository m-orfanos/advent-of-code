import { assertEquals } from "jsr:@std/assert";
import { solve } from "../solve.ts";

Deno.test(`2024 D11 P02`, async () => {
  const ans = await solve("2024", "11", "02");
  assertEquals(ans, 219838428124832);
});
