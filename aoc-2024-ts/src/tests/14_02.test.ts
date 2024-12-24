import { assertEquals } from "jsr:@std/assert";
import { solve } from "./solve.ts";

Deno.test(`2024 D14 P02`, async () => {
  const ans = await solve("14", "02");
  assertEquals(ans, 8087);
});
