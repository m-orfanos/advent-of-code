import { assertEquals } from "jsr:@std/assert";
import { solve } from "../solve.ts";

Deno.test(`2024 D01 P02`, async () => {
  const ans = await solve("2024", "01", "02");
  assertEquals(ans, 22588371);
});
