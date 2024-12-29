import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D21 P02`, async () => {
  const ans = await solve("21", "02");
  assertEquals(ans, 307055584161760);
});
